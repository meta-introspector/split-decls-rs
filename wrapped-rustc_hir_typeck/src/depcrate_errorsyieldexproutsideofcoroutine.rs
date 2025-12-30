// Generated macro for YieldExprOutsideOfCoroutine (struct)
macro_rules! Depcrate_errorsYieldExprOutsideOfCoroutine {
() => {
// Module: crate::errors
// Provides: {"YieldExprOutsideOfCoroutine"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (hir_typeck_yield_expr_outside_of_coroutine , code = E0627)] pub (crate) struct YieldExprOutsideOfCoroutine { # [primary_span] pub span : Span , }
};
}
