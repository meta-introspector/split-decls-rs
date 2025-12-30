// Generated macro for is_inside_unawaited_async_block (function)
macro_rules! Depcrate_loops_infinite_loopis_inside_unawaited_async_block {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"is_inside_unawaited_async_block"}
// Dependencies: {}
# [doc = " Check if the given expression is inside an async block that is not being awaited."] # [doc = " This helps avoid false positives when async blocks are spawned or assigned to variables."] fn is_inside_unawaited_async_block (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { let current_hir_id = expr . hir_id ; for (_ , parent_node) in cx . tcx . hir_parent_iter (current_hir_id) { if let Node :: Expr (Expr { kind : ExprKind :: Closure (Closure { kind : ClosureKind :: Coroutine (CoroutineKind :: Desugared (CoroutineDesugaring :: Async , CoroutineSource :: Block | CoroutineSource :: Closure ,)) , .. }) , .. }) = parent_node { return ! is_async_block_awaited (cx , expr) ; } } false }
};
}
