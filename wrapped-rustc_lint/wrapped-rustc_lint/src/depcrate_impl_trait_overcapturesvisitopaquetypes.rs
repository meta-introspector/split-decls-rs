// Generated macro for VisitOpaqueTypes (struct)
macro_rules! Depcrate_impl_trait_overcapturesVisitOpaqueTypes {
() => {
// Module: crate::impl_trait_overcaptures
// Provides: {"VisitOpaqueTypes"}
// Dependencies: {}
struct VisitOpaqueTypes < 'tcx , VarFn , OutlivesFn > { tcx : TyCtxt < 'tcx > , parent_def_id : LocalDefId , in_scope_parameters : FxIndexMap < DefId , ParamKind > , variances : LazyCell < FxHashMap < DefId , ty :: Variance > , VarFn > , outlives_env : LazyCell < OutlivesEnvironment < 'tcx > , OutlivesFn > , seen : FxIndexSet < LocalDefId > , }
};
}
