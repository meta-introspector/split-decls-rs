// Generated macro for impl_534 (impl)
macro_rules! Depcrate_vectorimpl_534 {
() => {
// Module: crate::vector
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'a , A : Clone > IntoIterator for & 'a Vector < A > { type Item = & 'a A ; type IntoIter = Iter < 'a , A > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
};
}
