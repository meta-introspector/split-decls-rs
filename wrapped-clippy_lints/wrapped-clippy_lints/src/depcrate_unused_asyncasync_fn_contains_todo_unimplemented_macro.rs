// Generated macro for async_fn_contains_todo_unimplemented_macro (function)
macro_rules! Depcrate_unused_asyncasync_fn_contains_todo_unimplemented_macro {
() => {
// Module: crate::unused_async
// Provides: {"async_fn_contains_todo_unimplemented_macro"}
// Dependencies: {}
fn async_fn_contains_todo_unimplemented_macro (cx : & LateContext < '_ > , body : & Body < '_ >) -> bool { if let ExprKind :: Closure (closure) = body . value . kind && let ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , _)) = closure . kind && let body = cx . tcx . hir_body (closure . body) && let ExprKind :: Block (block , _) = body . value . kind && block . stmts . is_empty () && let Some (expr) = block . expr && let ExprKind :: DropTemps (inner) = expr . kind { return is_todo_unimplemented_stub (cx , inner) ; } false }
};
}
