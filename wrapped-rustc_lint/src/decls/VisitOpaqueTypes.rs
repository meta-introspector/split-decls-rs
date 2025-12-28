macro_rules! deps {
    () => {
        ParamKind!();
    };
}

macro_rules! VisitOpaqueTypes {
    () => {
        deps!();
        struct VisitOpaqueTypes < 'tcx , VarFn , OutlivesFn > { tcx : TyCtxt < 'tcx > , parent_def_id : LocalDefId , in_scope_parameters : FxIndexMap < DefId , ParamKind > , variances : LazyCell < FxHashMap < DefId , ty :: Variance > , VarFn > , outlives_env : LazyCell < OutlivesEnvironment < 'tcx > , OutlivesFn > , seen : FxIndexSet < LocalDefId > , }
    };
}

VisitOpaqueTypes!();