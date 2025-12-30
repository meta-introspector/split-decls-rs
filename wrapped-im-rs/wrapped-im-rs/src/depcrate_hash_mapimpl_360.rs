// Generated macro for impl_360 (impl)
macro_rules! Depcrate_hash_mapimpl_360 {
() => {
// Module: crate::hash::map
// Provides: {"impl_360"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a HashMap < K , V , S > where K : Hash + Eq , S : BuildHasher , { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
