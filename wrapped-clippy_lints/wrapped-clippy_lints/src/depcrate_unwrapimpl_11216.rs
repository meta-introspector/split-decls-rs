// Generated macro for impl_11216 (impl)
macro_rules! Depcrate_unwrapimpl_11216 {
() => {
// Module: crate::unwrap
// Provides: {"impl_11216"}
// Dependencies: {}
# [doc = " Identical to derived impl, but ignores `span` on [`Local::WithFieldAccess`]"] impl PartialEq for Local { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: WithFieldAccess { local_id : self_local_id , field_indices : self_field_indices , .. } , Self :: WithFieldAccess { local_id : other_local_id , field_indices : other_field_indices , .. } ,) => self_local_id == other_local_id && self_field_indices == other_field_indices , (Self :: Pure { local_id : self_local_id , } , Self :: Pure { local_id : other_local_id , } ,) => self_local_id == other_local_id , _ => false , } } }
};
}
