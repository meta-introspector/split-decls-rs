// Generated macro for suggest_bool_comparison (function)
macro_rules! Depcrate_bool_comparisonsuggest_bool_comparison {
() => {
// Module: crate::bool_comparison
// Provides: {"suggest_bool_comparison"}
// Dependencies: {}
fn suggest_bool_comparison < 'a , 'tcx > (cx : & LateContext < 'tcx > , span : Span , expr : & Expr < '_ > , mut app : Applicability , message : & 'static str , conv_hint : impl FnOnce (Sugg < 'a >) -> Sugg < 'a > ,) { let hint = Sugg :: hir_with_context (cx , expr , span . ctxt () , ".." , & mut app) ; span_lint_and_sugg (cx , BOOL_COMPARISON , span , message , "try" , conv_hint (hint) . into_string () , app ,) ; }
};
}
