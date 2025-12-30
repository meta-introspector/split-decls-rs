// Generated macro for detect_option_match (function)
macro_rules! Depcrate_option_if_let_elsedetect_option_match {
() => {
// Module: crate::option_if_let_else
// Provides: {"detect_option_match"}
// Dependencies: {}
fn detect_option_match < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < 'tcx >) -> Option < OptionOccurrence > { if let ExprKind :: Match (ex , arms , MatchSource :: Normal) = expr . kind && ! cx . typeck_results () . expr_ty (expr) . is_unit () && let Some ((let_pat , if_then , if_else)) = try_convert_match (cx , arms) { try_get_option_occurrence (cx , expr . span . ctxt () , let_pat , ex , if_then , if_else) } else { None } }
};
}
