// Generated macro for impl_376 (impl)
macro_rules! Depcrate_autoderefimpl_376 {
() => {
// Module: crate::autoderef
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'table , 'db > Autoderef < 'table , 'db , usize > { pub (crate) fn new_no_tracking (table : & 'table mut InferenceTable < 'db > , ty : Ty , explicit : bool , use_receiver_trait : bool ,) -> Self { let ty = table . resolve_ty_shallow (& ty) ; Autoderef { table , ty , at_start : true , steps : 0 , explicit , use_receiver_trait } } }
};
}
