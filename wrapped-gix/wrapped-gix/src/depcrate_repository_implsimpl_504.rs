// Generated macro for impl_504 (impl)
macro_rules! Depcrate_repository_implsimpl_504 {
() => {
// Module: crate::repository::impls
// Provides: {"impl_504"}
// Dependencies: {}
impl gix_object :: Exists for crate :: Repository { fn exists (& self , id : & gix_hash :: oid) -> bool { if id == ObjectId :: empty_tree (self . object_hash ()) { return true ; } self . objects . exists (id) } }
};
}
