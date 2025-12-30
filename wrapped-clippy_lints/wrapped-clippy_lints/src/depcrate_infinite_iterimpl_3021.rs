// Generated macro for impl_3021 (impl)
macro_rules! Depcrate_infinite_iterimpl_3021 {
() => {
// Module: crate::infinite_iter
// Provides: {"impl_3021"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for InfiniteIter { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (lint , msg) = match complete_infinite_iter (cx , expr) { Infinite => (INFINITE_ITER , "infinite iteration detected") , MaybeInfinite => (MAYBE_INFINITE_ITER , "possible infinite iteration detected") , Finite => { return ; } , } ; span_lint (cx , lint , expr . span , msg) ; } }
};
}
