// Generated macro for is_async_closure (function)
macro_rules! Depcrate_redundant_closure_callis_async_closure {
() => {
// Module: crate::redundant_closure_call
// Provides: {"is_async_closure"}
// Dependencies: {}
# [doc = " Checks if the body is owned by an async closure."] # [doc = " Returns true for `async || whatever_expression`, but false for `|| async { whatever_expression"] # [doc = " }`."] fn is_async_closure (body : & hir :: Body < '_ >) -> bool { if let ExprKind :: Closure (innermost_closure_generated_by_desugar) = body . value . kind && let ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Closure)) = innermost_closure_generated_by_desugar . kind { true } else { false } }
};
}
