// Generated macro for impl_203 (impl)
macro_rules! Depcrate_lru_cacheimpl_203 {
() => {
// Module: crate::lru_cache
// Provides: {"impl_203"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a mut LruCache < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
};
}
