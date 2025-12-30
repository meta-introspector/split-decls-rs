// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a mut CLruCache < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = CLruCacheIterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> CLruCacheIterMut < 'a , K , V > { self . iter_mut () } }
};
}
