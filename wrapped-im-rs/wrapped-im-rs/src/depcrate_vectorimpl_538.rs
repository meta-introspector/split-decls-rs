// Generated macro for impl_538 (impl)
macro_rules! Depcrate_vectorimpl_538 {
() => {
// Module: crate::vector
// Provides: {"impl_538"}
// Dependencies: {}
impl < 'a , A : Clone > From < & 'a [A] > for Vector < A > { fn from (slice : & [A]) -> Self { slice . iter () . cloned () . collect () } }
};
}
