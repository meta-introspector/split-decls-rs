// Generated macro for check (function)
macro_rules! Depcrate_methods_manual_is_variant_andcheck {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , map_recv : & Expr < '_ > , map_arg : & Expr < '_ > , map_span : Span , msrv : Msrv ,) { if expr . span . from_expansion () { return ; } let is_option = cx . typeck_results () . expr_ty (map_recv) . is_diag_item (cx , sym :: Option) ; let is_result = cx . typeck_results () . expr_ty (map_recv) . is_diag_item (cx , sym :: Result) ; if ! is_option && ! is_result { return ; } if ! cx . typeck_results () . expr_ty (expr) . is_bool () { return ; } if ! msrv . meets (cx , msrvs :: OPTION_RESULT_IS_VARIANT_AND) { return ; } let lint_msg = if is_option { "called `map(<f>).unwrap_or_default()` on an `Option` value" } else { "called `map(<f>).unwrap_or_default()` on a `Result` value" } ; let suggestion = if is_option { "is_some_and" } else { "is_ok_and" } ; span_lint_and_sugg (cx , MANUAL_IS_VARIANT_AND , expr . span . with_lo (map_span . lo ()) , lint_msg , "use" , format ! ("{}({})" , suggestion , snippet (cx , map_arg . span , "..")) , Applicability :: MachineApplicable ,) ; }
};
}
