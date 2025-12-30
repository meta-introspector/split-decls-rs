// Generated macro for is_expr_async_block (function)
macro_rules! Depcrateis_expr_async_block {
() => {
// Module: crate
// Provides: {"is_expr_async_block"}
// Dependencies: {}
# [doc = " Checks if the expression is an async block (i.e., `async { ... }`)."] pub fn is_expr_async_block (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Closure (Closure { kind : hir :: ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Block)) , .. })) }
};
}
