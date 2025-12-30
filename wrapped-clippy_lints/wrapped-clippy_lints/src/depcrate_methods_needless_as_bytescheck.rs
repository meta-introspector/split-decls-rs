// Generated macro for check (function)
macro_rules! Depcrate_methods_needless_as_bytescheck {
() => {
// Module: crate::methods::needless_as_bytes
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ > , prev_method : Symbol , method : Symbol , prev_recv : & Expr < '_ > , span : Span) { let ty1 = cx . typeck_results () . expr_ty_adjusted (prev_recv) . peel_refs () ; if ty1 . is_lang_item (cx , LangItem :: String) || ty1 . is_str () { let mut app = Applicability :: MachineApplicable ; let sugg = Sugg :: hir_with_context (cx , prev_recv , span . ctxt () , ".." , & mut app) ; span_lint_and_sugg (cx , NEEDLESS_AS_BYTES , span , format ! ("needless call to `{prev_method}`") , format ! ("`{method}()` can be called directly on strings") , format ! ("{sugg}.{method}()") , app ,) ; } }
};
}
