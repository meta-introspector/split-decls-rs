// Generated macro for emit_return_lint (function)
macro_rules! Depcrate_returns_needless_returnemit_return_lint {
() => {
// Module: crate::returns::needless_return
// Provides: {"emit_return_lint"}
// Dependencies: {}
fn emit_return_lint (cx : & LateContext < '_ > , lint_span : Span , ret_span : Span , semi_spans : Vec < Span > , replacement : & RetReplacement < '_ > , at : HirId ,) { span_lint_hir_and_then (cx , NEEDLESS_RETURN , at , lint_span , "unneeded `return` statement" , | diag | { let suggestions = std :: iter :: once ((ret_span , replacement . to_string ())) . chain (semi_spans . into_iter () . map (| span | (span , String :: new ()))) . collect () ; diag . multipart_suggestion_verbose (replacement . sugg_help () , suggestions , replacement . applicability ()) ; } ,) ; }
};
}
