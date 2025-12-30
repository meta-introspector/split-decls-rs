// Generated macro for check_replace_option_with_none (function)
macro_rules! Depcrate_mem_replacecheck_replace_option_with_none {
() => {
// Module: crate::mem_replace
// Provides: {"check_replace_option_with_none"}
// Dependencies: {}
fn check_replace_option_with_none (cx : & LateContext < '_ > , src : & Expr < '_ > , dest : & Expr < '_ > , expr_span : Span) -> bool { if is_none_expr (cx , src) { let sugg_expr = peel_ref_operators (cx , dest) ; let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , MEM_REPLACE_OPTION_WITH_NONE , expr_span , "replacing an `Option` with `None`" , "consider `Option::take()` instead" , format ! ("{}.take()" , Sugg :: hir_with_context (cx , sugg_expr , expr_span . ctxt () , "" , & mut applicability) . maybe_paren ()) , applicability ,) ; true } else { false } }
};
}
