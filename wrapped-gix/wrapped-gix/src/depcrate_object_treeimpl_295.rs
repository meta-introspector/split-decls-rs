// Generated macro for impl_295 (impl)
macro_rules! Depcrate_object_treeimpl_295 {
() => {
// Module: crate::object::tree
// Provides: {"impl_295"}
// Dependencies: {}
# [doc = " Initialization"] impl < 'repo > Tree < 'repo > { # [doc = " Obtain a tree instance by handing in all components that it is made up of."] pub fn from_data (id : impl Into < ObjectId > , data : Vec < u8 > , repo : & 'repo crate :: Repository) -> Self { Tree { id : id . into () , data , repo , } } }
};
}
