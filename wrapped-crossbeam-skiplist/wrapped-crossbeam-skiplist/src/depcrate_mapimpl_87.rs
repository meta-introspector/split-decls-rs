// Generated macro for impl_87 (impl)
macro_rules! Depcrate_mapimpl_87 {
() => {
// Module: crate::map
// Provides: {"impl_87"}
// Dependencies: {}
impl < K , V > IntoIterator for SkipMap < K , V > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; fn into_iter (self) -> IntoIter < K , V > { IntoIter { inner : self . inner . into_iter () , } } }
};
}
