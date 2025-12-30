// Generated macro for check_flatten (function)
macro_rules! Depcrate_methods_lines_filter_map_okcheck_flatten {
() => {
// Module: crate::methods::lines_filter_map_ok
// Provides: {"check_flatten"}
// Dependencies: {}
pub (super) fn check_flatten (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , call_span : Span , msrv : Msrv) { if cx . ty_based_def (expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && cx . typeck_results () . expr_ty_adjusted (recv) . is_diag_item (cx , sym :: IoLines) && msrv . meets (cx , msrvs :: MAP_WHILE) { emit (cx , recv , "flatten" , call_span) ; } }
};
}
