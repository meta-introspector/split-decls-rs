// Generated macro for GenericParamAndBoundVarCollector (struct)
macro_rules! Depcrate_hir_ty_lowering_boundsGenericParamAndBoundVarCollector {
() => {
// Module: crate::hir_ty_lowering::bounds
// Provides: {"GenericParamAndBoundVarCollector"}
// Dependencies: {}
struct GenericParamAndBoundVarCollector < 'a , 'tcx > { cx : & 'a dyn HirTyLowerer < 'tcx > , params : FxIndexSet < u32 > , vars : FxIndexSet < DefId > , depth : ty :: DebruijnIndex , }
};
}
