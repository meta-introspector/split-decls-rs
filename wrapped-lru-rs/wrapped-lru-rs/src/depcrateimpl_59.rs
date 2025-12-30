// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl < K : Hash + Eq , V > IntoIterator for LruCache < K , V > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; fn into_iter (self) -> IntoIter < K , V > { IntoIter { cache : self } } }
};
}
