// Generated macro for check_replace_with_default (function)
macro_rules! Depcrate_mem_replacecheck_replace_with_default {
() => {
// Module: crate::mem_replace
// Provides: {"check_replace_with_default"}
// Dependencies: {}
fn check_replace_with_default (cx : & LateContext < '_ > , src : & Expr < '_ > , dest : & Expr < '_ > , expr : & Expr < '_ > , msrv : Msrv ,) -> bool { if is_expr_used_or_unified (cx . tcx , expr) && let expr_type = cx . typeck_results () . expr_ty_adjusted (src) && ! is_non_aggregate_primitive_type (expr_type) && is_default_equivalent (cx , src) && ! expr . span . in_external_macro (cx . tcx . sess . source_map ()) && let Some (top_crate) = std_or_core (cx) && msrv . meets (cx , msrvs :: MEM_TAKE) { span_lint_and_then (cx , MEM_REPLACE_WITH_DEFAULT , expr . span , format ! ("replacing a value of type `T` with `T::default()` is better expressed using `{top_crate}::mem::take`") , | diag | { if ! expr . span . from_expansion () { let mut applicability = Applicability :: MachineApplicable ; let (dest_snip , _) = snippet_with_context (cx , dest . span , expr . span . ctxt () , "" , & mut applicability) ; let suggestion = format ! ("{top_crate}::mem::take({dest_snip})") ; diag . span_suggestion (expr . span , "consider using" , suggestion , applicability) ; } } ,) ; true } else { false } }
};
}
