use std::collections::HashMap;

use crate::{ast::{Prog, ProgAtom, ProgAtom::*}, aexp::AExp, aexp::AExp::*, bexp::BExp::*, common::VarName, bexp::BExp};
use std::fs::read_to_string;

/// This struct represents a memory configuration. Each variable is assigned an `i32` via a `HashMap`; if there is no entry in the `HashMap`, then the assignment is `0`.
#[derive(Debug)]
pub struct MemConfig(HashMap<VarName, i32>);

impl MemConfig {
    pub fn new() -> Self { Self(HashMap::new()) }
    
    /// Read operation (with `0` as default value)
    pub fn lookup(&self, x: &VarName) -> i32 {
        let MemConfig(map) = self;
        return *map.get(x).unwrap_or(&(0));
    }

    /// Write operation
    pub fn assign(&mut self, x: &VarName, n: i32) {
        let MemConfig(map) = self;
        map.insert(x.clone(), n);
    }
}

/// Input: Program + Assignment to "x" variable
/// Output:
/// - If `p` terminates: Assignment to "y" variable
/// - If `p` diverges: This function diverges, too 
pub fn eval(p: &Prog, input: i32) -> i32 {
    let mut mem = MemConfig::new();
    mem.assign(&VarName::new("x"), input);
    mem = eval_prog(p, mem);
    mem.lookup(&VarName::new("z"))
}

/// Evaluate program on given memory configuration. This functin may diverge.
pub fn eval_prog(p: &Prog, mem: MemConfig) -> MemConfig {
    let Prog::Prog(ps) = p;
    ps.iter().fold(mem, |mem,p| eval_prog_atom(p, mem))
}

/// Evaluate atomic program on given memory configuration. This function may diverge.
pub fn eval_prog_atom(p: &ProgAtom, mut mem: MemConfig) -> MemConfig {
    match p {
        Skip => { mem }
        Assign(x, a) => {
            let n = eval_aexp(a, &mem);
            mem.assign(x, n);
            mem
        }
        Cond(b, p1, p2) => {
            let result = eval_bexp(b, &mem);
            return if result {
                eval_prog(p1, mem)
            } else {
                eval_prog(p2, mem)
            }
        }
        While(b, p) => {
            //TODO maybe fix later clone mem
            while eval_bexp(b,&mem) {
                mem = eval_prog(p,mem);
            }
            return mem;
        }
    }
}

/// Evaluate arithmetic expression on given memory configuration. This function always returns.
pub fn eval_aexp(a: &AExp, mem: &MemConfig) -> i32 {
    match a {
        Num(n) => { *n }
        Var(x) => { mem.lookup(x) }
        Add(a1, a2) => { eval_aexp(a1, mem) + eval_aexp(a2, mem) }
        Mul(a1, a2) => { eval_aexp(a1, mem) * eval_aexp(a2, mem) }
    }
}

/// Evaluate boolean expression on given memory configuration. This function always returns.
pub fn eval_bexp(a: &BExp, mem: &MemConfig) -> bool {
    return match a {
        LessEq(a1, a2) => {
            eval_aexp(a1, mem) <= eval_aexp(a2, mem)
        }
        Or(b1,b2) => {
            eval_bexp(b1, mem) || eval_bexp(b2, mem)
        }
        And(b1,b2) => {
            eval_bexp(b1, mem) && eval_bexp(b2, mem)
        }
        Neg(b1) => {
            !eval_bexp(b1, mem)
        }
    }
}