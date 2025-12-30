// Generated macro for impl_9 (impl)
macro_rules! Depcrate_async_closuresimpl_9 {
() => {
// Module: crate::async_closures
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AsyncClosureUsage { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { let hir :: ExprKind :: Closure (& hir :: Closure { body , kind : hir :: ClosureKind :: Closure , fn_decl_span , .. }) = expr . kind else { return ; } ; let mut body = cx . tcx . hir_body (body) . value ; while let hir :: ExprKind :: Block (& hir :: Block { stmts : [] , expr : Some (tail) , .. } , None) = body . kind { body = tail ; } let hir :: ExprKind :: Closure (& hir :: Closure { kind : hir :: ClosureKind :: Coroutine (hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , hir :: CoroutineSource :: Block ,)) , fn_decl_span : async_decl_span , .. }) = body . kind else { return ; } ; let deletion_span = cx . tcx . sess . source_map () . span_extend_while_whitespace (async_decl_span) ; cx . tcx . emit_node_span_lint (CLOSURE_RETURNING_ASYNC_BLOCK , expr . hir_id , fn_decl_span , ClosureReturningAsyncBlock { async_decl_span , sugg : AsyncClosureSugg { deletion_span , insertion_span : fn_decl_span . shrink_to_lo () , } , } ,) ; } }
};
}
