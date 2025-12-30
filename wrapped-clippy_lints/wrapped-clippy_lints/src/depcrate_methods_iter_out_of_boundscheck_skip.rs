// Generated macro for check_skip (function)
macro_rules! Depcrate_methods_iter_out_of_boundscheck_skip {
() => {
// Module: crate::methods::iter_out_of_bounds
// Provides: {"check_skip"}
// Dependencies: {}
pub (super) fn check_skip < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , recv : & 'tcx Expr < 'tcx > , arg : & 'tcx Expr < 'tcx > ,) { check (cx , expr , recv , arg , "this `.skip()` call skips more items than the iterator will produce" , "this operation is useless and will create an empty iterator" ,) ; }
};
}
