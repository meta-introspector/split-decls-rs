// Generated macro for get_concrete_opaque_type (function)
macro_rules! Depcrate_region_infer_opaque_typesget_concrete_opaque_type {
() => {
// Module: crate::region_infer::opaque_types
// Provides: {"get_concrete_opaque_type"}
// Dependencies: {}
fn get_concrete_opaque_type < 'tcx > (concrete_opaque_types : & ConcreteOpaqueTypes < 'tcx > , def_id : LocalDefId ,) -> Option < EarlyBinder < 'tcx , OpaqueHiddenType < 'tcx > > > { concrete_opaque_types . 0 . get (& def_id) . map (| ty | EarlyBinder :: bind (* ty)) }
};
}
