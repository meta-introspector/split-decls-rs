// Generated macro for impl_144 (impl)
macro_rules! Depcrate_index_mapimpl_144 {
() => {
// Module: crate::index_map
// Provides: {"impl_144"}
// Dependencies: {}
impl < K , V , S , const N : usize > IntoIterator for IndexMap < K , V , S , N > { type Item = (K , V) ; type IntoIter = IntoIter < K , V , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { entries : self . core . entries , } } }
};
}
