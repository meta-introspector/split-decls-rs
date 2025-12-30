// Generated macro for get_expr_snippet_with_type_certainty (function)
macro_rules! Depcrate_unit_types_unit_argget_expr_snippet_with_type_certainty {
() => {
// Module: crate::unit_types::unit_arg
// Provides: {"get_expr_snippet_with_type_certainty"}
// Dependencies: {}
fn get_expr_snippet_with_type_certainty < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < 'tcx > ,) -> Option < MaybeTypeUncertain < 'tcx > > { get_expr_snippet (cx , expr) . map (| snip | { if ! expr_type_is_certain (cx , expr) && ! is_block_with_no_expr (expr) { MaybeTypeUncertain :: Uncertain (snip) } else { MaybeTypeUncertain :: Certain (snip) } }) }
};
}
