// Generated macro for impl_375 (impl)
macro_rules! Depcrate_autoderefimpl_375 {
() => {
// Module: crate::autoderef
// Provides: {"impl_375"}
// Dependencies: {}
impl < 'table , 'db > Autoderef < 'table , 'db > { pub (crate) fn new (table : & 'table mut InferenceTable < 'db > , ty : Ty , explicit : bool , use_receiver_trait : bool ,) -> Self { let ty = table . resolve_ty_shallow (& ty) ; Autoderef { table , ty , at_start : true , steps : Vec :: new () , explicit , use_receiver_trait } } pub (crate) fn steps (& self) -> & [(AutoderefKind , Ty)] { & self . steps } }
};
}
