// Generated macro for impl_109 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_109 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_109"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a LinkedHashMap < K , V , S > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
};
}
