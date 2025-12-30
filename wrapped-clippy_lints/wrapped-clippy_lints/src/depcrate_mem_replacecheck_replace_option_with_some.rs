// Generated macro for check_replace_option_with_some (function)
macro_rules! Depcrate_mem_replacecheck_replace_option_with_some {
() => {
// Module: crate::mem_replace
// Provides: {"check_replace_option_with_some"}
// Dependencies: {}
fn check_replace_option_with_some (cx : & LateContext < '_ > , src : & Expr < '_ > , dest : & Expr < '_ > , expr_span : Span , msrv : Msrv ,) -> bool { if let Some (src_arg) = as_some_expr (cx , src) && msrv . meets (cx , msrvs :: OPTION_REPLACE) { let sugg_expr = peel_ref_operators (cx , dest) ; let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , MEM_REPLACE_OPTION_WITH_SOME , expr_span , "replacing an `Option` with `Some(..)`" , "consider `Option::replace()` instead" , format ! ("{}.replace({})" , Sugg :: hir_with_context (cx , sugg_expr , expr_span . ctxt () , "_" , & mut applicability) . maybe_paren () , snippet_with_applicability (cx , src_arg . span , "_" , & mut applicability)) , applicability ,) ; true } else { false } }
};
}
