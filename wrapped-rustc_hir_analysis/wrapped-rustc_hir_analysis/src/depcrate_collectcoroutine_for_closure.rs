// Generated macro for coroutine_for_closure (function)
macro_rules! Depcrate_collectcoroutine_for_closure {
() => {
// Module: crate::collect
// Provides: {"coroutine_for_closure"}
// Dependencies: {}
fn coroutine_for_closure (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> DefId { let & rustc_hir :: Closure { kind : hir :: ClosureKind :: CoroutineClosure (_) , body , .. } = tcx . hir_node_by_def_id (def_id) . expect_closure () else { bug ! () } ; let & hir :: Expr { kind : hir :: ExprKind :: Closure (& rustc_hir :: Closure { def_id , kind : hir :: ClosureKind :: Coroutine (_) , .. }) , .. } = tcx . hir_body (body) . value else { bug ! () } ; def_id . to_def_id () }
};
}
