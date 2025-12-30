// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < 'a , K : Hash + Eq , V , S : BuildHasher > IntoIterator for & 'a LruCache < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
