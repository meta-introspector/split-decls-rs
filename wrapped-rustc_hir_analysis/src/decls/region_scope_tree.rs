macro_rules! deps {
    () => {
        Context!();
        ScopeResolutionVisitor!();
    };
}

macro_rules! region_scope_tree {
    () => {
        deps!();
        # [doc = " Per-body `region::ScopeTree`. The `DefId` should be the owner `DefId` for the body;"] # [doc = " in the case of closures, this will be redirected to the enclosing function."] # [doc = ""] # [doc = " Performance: This is a query rather than a simple function to enable"] # [doc = " re-use in incremental scenarios. We may sometimes need to rerun the"] # [doc = " type checker even when the HIR hasn't changed, and in those cases"] # [doc = " we can avoid reconstructing the region scope tree."] pub (crate) fn region_scope_tree (tcx : TyCtxt < '_ > , def_id : DefId) -> & ScopeTree { let typeck_root_def_id = tcx . typeck_root_def_id (def_id) ; if typeck_root_def_id != def_id { return tcx . region_scope_tree (typeck_root_def_id) ; } let scope_tree = if let Some (body) = tcx . hir_maybe_body_owned_by (def_id . expect_local ()) { let mut visitor = ScopeResolutionVisitor { tcx , scope_tree : ScopeTree :: default () , cx : Context { parent : None , var_parent : (None , ScopeCompatibility :: FutureCompatible) } , extended_super_lets : Default :: default () , } ; visitor . scope_tree . root_body = Some (body . value . hir_id) ; visitor . visit_body (& body) ; visitor . scope_tree } else { ScopeTree :: default () } ; tcx . arena . alloc (scope_tree) }
    };
}

region_scope_tree!();