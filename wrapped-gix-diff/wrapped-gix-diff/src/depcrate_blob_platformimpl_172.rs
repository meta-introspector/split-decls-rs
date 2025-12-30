// Generated macro for impl_172 (impl)
macro_rules! Depcrate_blob_platformimpl_172 {
() => {
// Module: crate::blob::platform
// Provides: {"impl_172"}
// Dependencies: {}
impl std :: hash :: Hash for CacheKey { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { if self . use_id { self . id . hash (state) ; self . is_link . hash (state) ; } else { self . location . hash (state) ; } } }
};
}
