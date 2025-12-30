// Generated macro for impl_199 (impl)
macro_rules! Depcrate_lru_cacheimpl_199 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_199"}
// Dependencies: {}
impl < K : Hash + Eq + Clone , V : Clone , S : BuildHasher + Clone > Clone for LruCache < K , V , S > { # [inline] fn clone (& self) -> Self { LruCache { map : self . map . clone () , max_size : self . max_size , } } }
};
}
