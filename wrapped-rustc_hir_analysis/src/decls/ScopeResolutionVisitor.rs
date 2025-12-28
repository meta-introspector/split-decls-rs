macro_rules! deps {
    () => {
        Context!();
        ExtendedTemporaryScope!();
    };
}

macro_rules! ScopeResolutionVisitor {
    () => {
        deps!();
        struct ScopeResolutionVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , scope_tree : ScopeTree , cx : Context , extended_super_lets : FxHashMap < hir :: ItemLocalId , ExtendedTemporaryScope > , }
    };
}

ScopeResolutionVisitor!()