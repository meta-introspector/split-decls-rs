// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , K : Hash + Eq , V , S : BuildHasher > IntoIterator for & 'a mut LruCache < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
};
}
