// Generated macro for impl_489 (impl)
macro_rules! Depcrate_await_holding_invalidimpl_489 {
() => {
// Module: crate::await_holding_invalid
// Provides: {"impl_489"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for AwaitHolding { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < 'tcx >) { if let hir :: ExprKind :: Closure (hir :: Closure { kind : hir :: ClosureKind :: Coroutine (hir :: CoroutineKind :: Desugared (hir :: CoroutineDesugaring :: Async , _)) , def_id , .. }) = expr . kind && let Some (coroutine_layout) = cx . tcx . mir_coroutine_witnesses (* def_id) { self . check_interior_types (cx , coroutine_layout) ; } } }
};
}
