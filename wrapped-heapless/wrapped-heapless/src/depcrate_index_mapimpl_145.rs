// Generated macro for impl_145 (impl)
macro_rules! Depcrate_index_mapimpl_145 {
() => {
// Module: crate::index_map
// Provides: {"impl_145"}
// Dependencies: {}
impl < 'a , K , V , S , const N : usize > IntoIterator for & 'a IndexMap < K , V , S , N > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
