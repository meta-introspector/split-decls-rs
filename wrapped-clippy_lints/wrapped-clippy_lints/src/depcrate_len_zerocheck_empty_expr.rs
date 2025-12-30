// Generated macro for check_empty_expr (function)
macro_rules! Depcrate_len_zerocheck_empty_expr {
() => {
// Module: crate::len_zero
// Provides: {"check_empty_expr"}
// Dependencies: {}
fn check_empty_expr (cx : & LateContext < '_ > , span : Span , lit1 : & Expr < '_ > , lit2 : & Expr < '_ > , op : & str) { if (is_empty_array (lit2) || is_empty_string (lit2)) && has_is_empty (cx , lit1) { let mut applicability = Applicability :: MachineApplicable ; let lit1 = peel_ref_operators (cx , lit1) ; let lit_str = Sugg :: hir_with_context (cx , lit1 , span . ctxt () , "_" , & mut applicability) . maybe_paren () ; span_lint_and_sugg (cx , COMPARISON_TO_EMPTY , span , "comparison to empty slice" , format ! ("using `{op}is_empty` is clearer and more explicit") , format ! ("{op}{lit_str}.is_empty()") , applicability ,) ; } }
};
}
