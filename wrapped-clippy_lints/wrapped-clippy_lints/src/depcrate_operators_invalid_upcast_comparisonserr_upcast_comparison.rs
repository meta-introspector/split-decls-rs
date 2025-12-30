// Generated macro for err_upcast_comparison (function)
macro_rules! Depcrate_operators_invalid_upcast_comparisonserr_upcast_comparison {
() => {
// Module: crate::operators::invalid_upcast_comparisons
// Provides: {"err_upcast_comparison"}
// Dependencies: {}
fn err_upcast_comparison (cx : & LateContext < '_ > , span : Span , expr : & Expr < '_ > , always : bool) { if let ExprKind :: Cast (cast_val , _) = expr . kind { let mut applicability = Applicability :: MachineApplicable ; let (cast_val_snip , _) = snippet_with_context (cx , cast_val . span , expr . span . ctxt () , "the expression" , & mut applicability ,) ; span_lint (cx , INVALID_UPCAST_COMPARISONS , span , format ! ("because of the numeric bounds on `{}` prior to casting, this expression is always {}" , cast_val_snip , if always { "true" } else { "false" } ,) ,) ; } }
};
}
