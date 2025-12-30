// Generated macro for impl_64 (impl)
macro_rules! Depcrate_ext_object_idimpl_64 {
() => {
// Module: crate::ext::object_id
// Provides: {"impl_64"}
// Dependencies: {}
impl ObjectIdExt for ObjectId { fn ancestors < Find > (self , find : Find) -> AncestorsIter < Find > where Find : gix_object :: Find , { gix_traverse :: commit :: Simple :: new (Some (self) , find) } fn attach (self , repo : & crate :: Repository) -> crate :: Id < '_ > { crate :: Id :: from_id (self , repo) } }
};
}
