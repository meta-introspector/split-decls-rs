// Generated macro for tests (module)
macro_rules! Depcrate_ir_atomic_rmw_optests {
() => {
// Module: crate::ir::atomic_rmw_op
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn roundtrip_parse () { for op in AtomicRmwOp :: all () { let roundtripped = format ! ("{op}") . parse :: < AtomicRmwOp > () . unwrap () ; assert_eq ! (* op , roundtripped) ; } } }
};
}
