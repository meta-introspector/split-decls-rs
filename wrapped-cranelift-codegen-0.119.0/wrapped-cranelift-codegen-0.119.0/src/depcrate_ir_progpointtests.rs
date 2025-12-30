// Generated macro for tests (module)
macro_rules! Depcrate_ir_progpointtests {
() => {
// Module: crate::ir::progpoint
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: entity :: EntityRef ; use alloc :: string :: ToString ; # [test] fn convert () { let i5 = Inst :: new (5) ; let b3 = Block :: new (3) ; let pp1 : ProgramPoint = i5 . into () ; let pp2 : ProgramPoint = b3 . into () ; assert_eq ! (pp1 . to_string () , "inst5") ; assert_eq ! (pp2 . to_string () , "block3") ; } }
};
}
