macro_rules! YieldExprOutsideOfCoroutine {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_yield_expr_outside_of_coroutine , code = E0627)] pub (crate) struct YieldExprOutsideOfCoroutine { # [primary_span] pub span : Span , }
    };
}

YieldExprOutsideOfCoroutine!();