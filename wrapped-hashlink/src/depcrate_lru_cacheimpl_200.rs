// Generated macro for impl_200 (impl)
macro_rules! Depcrate_lru_cacheimpl_200 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_200"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher > Extend < (K , V) > for LruCache < K , V , S > { # [inline] fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , iter : I) { for (k , v) in iter { self . insert (k , v) ; } } }
};
}
