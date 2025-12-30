// Generated macro for err_upcast_comparison (function)
macro_rules! Depcrate_invalid_upcast_comparisonserr_upcast_comparison {
() => {
// Module: crate::invalid_upcast_comparisons
// Provides: {"err_upcast_comparison"}
// Dependencies: {}
fn err_upcast_comparison (cx : & LateContext < '_ > , span : Span , expr : & Expr < '_ > , always : bool) { if let ExprKind :: Cast (cast_val , _) = expr . kind { span_lint (cx , INVALID_UPCAST_COMPARISONS , span , format ! ("because of the numeric bounds on `{}` prior to casting, this expression is always {}" , snippet (cx , cast_val . span , "the expression") , if always { "true" } else { "false" } ,) ,) ; } }
};
}
