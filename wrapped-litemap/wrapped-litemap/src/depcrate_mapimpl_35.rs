// Generated macro for impl_35 (impl)
macro_rules! Depcrate_mapimpl_35 {
() => {
// Module: crate::map
// Provides: {"impl_35"}
// Dependencies: {}
impl < K , V , S > IntoIterator for LiteMap < K , V , S > where S : StoreIntoIterator < K , V > , { type Item = (K , V) ; type IntoIter = S :: KeyValueIntoIter ; fn into_iter (self) -> Self :: IntoIter { self . values . lm_into_iter () } }
};
}
