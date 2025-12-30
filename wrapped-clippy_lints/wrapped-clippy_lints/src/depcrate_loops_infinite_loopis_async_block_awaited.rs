// Generated macro for is_async_block_awaited (function)
macro_rules! Depcrate_loops_infinite_loopis_async_block_awaited {
() => {
// Module: crate::loops::infinite_loop
// Provides: {"is_async_block_awaited"}
// Dependencies: {}
fn is_async_block_awaited (cx : & LateContext < '_ > , async_expr : & Expr < '_ >) -> bool { for (_ , parent_node) in cx . tcx . hir_parent_iter (async_expr . hir_id) { if let Node :: Expr (Expr { kind : ExprKind :: Match (_ , _ , hir :: MatchSource :: AwaitDesugar) , .. }) = parent_node { return true ; } } false }
};
}
