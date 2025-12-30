// Generated macro for impl_146 (impl)
macro_rules! Depcrate_index_mapimpl_146 {
() => {
// Module: crate::index_map
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'a , K , V , S , const N : usize > IntoIterator for & 'a mut IndexMap < K , V , S , N > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter_mut () } }
};
}
