// Generated macro for impl_238 (impl)
macro_rules! Depcrate_memoryimpl_238 {
() => {
// Module: crate::memory
// Provides: {"impl_238"}
// Dependencies: {}
impl < T > gix_object :: Exists for Proxy < T > where T : gix_object :: Exists , { fn exists (& self , id : & gix_hash :: oid) -> bool { self . memory . as_ref () . is_some_and (| map | map . borrow () . contains_key (id)) || self . inner . exists (id) } }
};
}
