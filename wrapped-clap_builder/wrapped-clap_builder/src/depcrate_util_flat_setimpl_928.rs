// Generated macro for impl_928 (impl)
macro_rules! Depcrate_util_flat_setimpl_928 {
() => {
// Module: crate::util::flat_set
// Provides: {"impl_928"}
// Dependencies: {}
impl < 's , T : PartialEq + Eq > IntoIterator for & 's FlatSet < T > { type Item = & 's T ; type IntoIter = std :: slice :: Iter < 's , T > ; fn into_iter (self) -> Self :: IntoIter { self . inner . iter () } }
};
}
