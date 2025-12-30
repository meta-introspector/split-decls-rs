// Generated macro for check_take (function)
macro_rules! Depcrate_methods_iter_out_of_boundscheck_take {
() => {
// Module: crate::methods::iter_out_of_bounds
// Provides: {"check_take"}
// Dependencies: {}
pub (super) fn check_take < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > , recv : & 'tcx Expr < 'tcx > , arg : & 'tcx Expr < 'tcx > ,) { check (cx , expr , recv , arg , "this `.take()` call takes more items than the iterator will produce" , "this operation is useless and the returned iterator will simply yield the same items" ,) ; }
};
}
