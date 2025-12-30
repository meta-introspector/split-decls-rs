// Generated macro for impl_110 (impl)
macro_rules! Depcrate_linked_hash_mapimpl_110 {
() => {
// Module: crate::linked_hash_map
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a mut LinkedHashMap < K , V , S > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
};
}
