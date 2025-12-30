// Generated macro for impl_11217 (impl)
macro_rules! Depcrate_unwrapimpl_11217 {
() => {
// Module: crate::unwrap
// Provides: {"impl_11217"}
// Dependencies: {}
impl Local { fn snippet (& self , cx : & LateContext < '_ >) -> Cow < 'static , str > { match * self { Self :: WithFieldAccess { span , .. } => snippet (cx . sess () , span , "_") , Self :: Pure { local_id } => cx . tcx . hir_name (local_id) . to_string () . into () , } } fn is_potentially_local_place (& self , place : & Place < '_ >) -> bool { match self { Self :: WithFieldAccess { local_id , field_indices , .. } => { is_potentially_local_place (* local_id , place) && place . projections . len () <= field_indices . len () && iter :: zip (& place . projections , field_indices . iter () . copied () . rev ()) . all (| (proj , field_idx) | { match proj . kind { ProjectionKind :: Field (f_idx , _) => f_idx == field_idx , _ => false , } }) } , Self :: Pure { local_id } => is_potentially_local_place (* local_id , place) , } } }
};
}
