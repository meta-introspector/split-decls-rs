// Generated macro for impl_354 (impl)
macro_rules! Depcrate_infer_region_constraintsimpl_354 {
() => {
// Module: crate::infer::region_constraints
// Provides: {"impl_354"}
// Dependencies: {}
impl < 'tcx > GenericKind < 'tcx > { pub fn to_ty (& self , tcx : TyCtxt < 'tcx >) -> Ty < 'tcx > { match * self { GenericKind :: Param (ref p) => p . to_ty (tcx) , GenericKind :: Placeholder (ref p) => Ty :: new_placeholder (tcx , * p) , GenericKind :: Alias (ref p) => p . to_ty (tcx) , } } }
};
}
