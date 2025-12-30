// Generated macro for impl_201 (impl)
macro_rules! Depcrate_lru_cacheimpl_201 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_201"}
// Dependencies: {}
impl < K , V , S > IntoIterator for LruCache < K , V , S > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (self) -> IntoIter < K , V > { self . map . into_iter () } }
};
}
