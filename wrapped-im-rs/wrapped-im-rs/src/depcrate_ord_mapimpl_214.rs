// Generated macro for impl_214 (impl)
macro_rules! Depcrate_ord_mapimpl_214 {
() => {
// Module: crate::ord::map
// Provides: {"impl_214"}
// Dependencies: {}
impl < K , V > IntoIterator for OrdMap < K , V > where K : Ord + Clone , V : Clone , { type Item = (K , V) ; type IntoIter = ConsumingIter < (K , V) > ; fn into_iter (self) -> Self :: IntoIter { ConsumingIter :: new (& self . root , self . size) } }
};
}
