// Generated macro for impl_218 (impl)
macro_rules! Depcrate_linear_mapimpl_218 {
() => {
// Module: crate::linear_map
// Provides: {"impl_218"}
// Dependencies: {}
impl < K , V , const N : usize > IntoIterator for LinearMap < K , V , N > where K : Eq , { type Item = (K , V) ; type IntoIter = IntoIter < K , V , N > ; fn into_iter (self) -> Self :: IntoIter { IntoIter { inner : self . buffer . into_iter () , } } }
};
}
