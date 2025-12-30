// Generated macro for impl_121 (impl)
macro_rules! Depcrate_setimpl_121 {
() => {
// Module: crate::set
// Provides: {"impl_121"}
// Dependencies: {}
impl < T > IntoIterator for SkipSet < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { inner : self . inner . into_iter () , } } }
};
}
