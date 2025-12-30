// Generated macro for impl_213 (impl)
macro_rules! Depcrate_ord_mapimpl_213 {
() => {
// Module: crate::ord::map
// Provides: {"impl_213"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a OrdMap < K , V > where K : Ord , { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
