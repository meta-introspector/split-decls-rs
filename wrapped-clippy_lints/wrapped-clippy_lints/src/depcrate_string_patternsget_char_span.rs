// Generated macro for get_char_span (function)
macro_rules! Depcrate_string_patternsget_char_span {
() => {
// Module: crate::string_patterns
// Provides: {"get_char_span"}
// Dependencies: {}
fn get_char_span < 'tcx > (cx : & '_ LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < Span > { if cx . typeck_results () . expr_ty_adjusted (expr) . is_char () && ! expr . span . from_expansion () && switch_to_eager_eval (cx , expr) { Some (expr . span) } else { None } }
};
}
