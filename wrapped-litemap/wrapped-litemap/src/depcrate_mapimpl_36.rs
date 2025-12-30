// Generated macro for impl_36 (impl)
macro_rules! Depcrate_mapimpl_36 {
() => {
// Module: crate::map
// Provides: {"impl_36"}
// Dependencies: {}
impl < 'a , K , V , S > IntoIterator for & 'a LiteMap < K , V , S > where S : StoreIterable < 'a , K , V > , { type Item = (& 'a K , & 'a V) ; type IntoIter = S :: KeyValueIter ; fn into_iter (self) -> Self :: IntoIter { self . values . lm_iter () } }
};
}
