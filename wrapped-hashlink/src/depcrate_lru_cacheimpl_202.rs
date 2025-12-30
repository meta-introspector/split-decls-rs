// Generated macro for impl_202 (impl)
macro_rules! Depcrate_lru_cacheimpl_202 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_202"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a LruCache < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
