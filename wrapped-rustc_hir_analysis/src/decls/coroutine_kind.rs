macro_rules! coroutine_kind {
    () => {
        fn coroutine_kind (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < hir :: CoroutineKind > { match tcx . hir_node_by_def_id (def_id) { Node :: Expr (& hir :: Expr { kind : hir :: ExprKind :: Closure (& rustc_hir :: Closure { kind : hir :: ClosureKind :: Coroutine (kind) , .. }) , .. }) => Some (kind) , _ => None , } }
    };
}

coroutine_kind!();