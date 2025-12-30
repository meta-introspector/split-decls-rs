// Generated macro for impl_535 (impl)
macro_rules! Depcrate_vectorimpl_535 {
() => {
// Module: crate::vector
// Provides: {"impl_535"}
// Dependencies: {}
impl < A : Clone > IntoIterator for Vector < A > { type Item = A ; type IntoIter = ConsumingIter < A > ; fn into_iter (self) -> Self :: IntoIter { ConsumingIter :: new (self) } }
};
}
