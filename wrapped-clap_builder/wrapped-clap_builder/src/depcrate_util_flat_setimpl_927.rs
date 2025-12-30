// Generated macro for impl_927 (impl)
macro_rules! Depcrate_util_flat_setimpl_927 {
() => {
// Module: crate::util::flat_set
// Provides: {"impl_927"}
// Dependencies: {}
impl < T : PartialEq + Eq > IntoIterator for FlatSet < T > { type Item = T ; type IntoIter = std :: vec :: IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { self . inner . into_iter () } }
};
}
