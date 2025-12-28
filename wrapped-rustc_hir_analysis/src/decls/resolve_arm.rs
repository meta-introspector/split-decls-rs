macro_rules! deps {
    () => {
        ScopeResolutionVisitor!();
    };
}

macro_rules! resolve_arm {
    () => {
        deps!();
        fn resolve_arm < 'tcx > (visitor : & mut ScopeResolutionVisitor < 'tcx > , arm : & 'tcx hir :: Arm < 'tcx >) { let prev_cx = visitor . cx ; visitor . enter_node_scope_with_dtor (arm . hir_id . local_id , true) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; resolve_pat (visitor , arm . pat) ; if let Some (guard) = arm . guard { visitor . enter_scope (Scope { local_id : arm . hir_id . local_id , data : ScopeData :: MatchGuard }) ; visitor . cx . var_parent = (visitor . cx . parent , ScopeCompatibility :: FutureCompatible) ; resolve_cond (visitor , guard) ; } resolve_expr (visitor , arm . body , false) ; visitor . cx = prev_cx ; }
    };
}

resolve_arm!();