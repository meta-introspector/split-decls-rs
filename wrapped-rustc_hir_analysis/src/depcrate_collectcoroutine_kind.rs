// Generated macro for coroutine_kind (function)
macro_rules! Depcrate_collectcoroutine_kind {
() => {
// Module: crate::collect
// Provides: {"coroutine_kind"}
// Dependencies: {}
fn coroutine_kind (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < hir :: CoroutineKind > { match tcx . hir_node_by_def_id (def_id) { Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure (& rustc_hir :: Closure { kind : hir :: ClosureKind :: Coroutine (kind) , .. }) , .. }) => Some (kind) , _ => None , } }
};
}
