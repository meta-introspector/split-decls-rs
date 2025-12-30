// Generated macro for impl_204 (impl)
macro_rules! Depcrate_lru_cacheimpl_204 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_204"}
// Dependencies: {}
impl < K , V , S > fmt :: Debug for LruCache < K , V , S > where K : fmt :: Debug , V : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_map () . entries (self . iter () . rev ()) . finish () } }
};
}
