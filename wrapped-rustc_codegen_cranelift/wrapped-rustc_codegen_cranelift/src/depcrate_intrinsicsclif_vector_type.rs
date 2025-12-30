// Generated macro for clif_vector_type (function)
macro_rules! Depcrate_intrinsicsclif_vector_type {
() => {
// Module: crate::intrinsics
// Provides: {"clif_vector_type"}
// Dependencies: {}
pub (crate) fn clif_vector_type < 'tcx > (tcx : TyCtxt < 'tcx > , layout : TyAndLayout < 'tcx >) -> Type { let (element , count) = match layout . backend_repr { BackendRepr :: SimdVector { element , count } => (element , count) , _ => unreachable ! () , } ; scalar_to_clif_type (tcx , element) . by (u32 :: try_from (count) . unwrap ()) . unwrap () }
};
}
