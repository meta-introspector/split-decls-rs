// Generated macro for impl_525 (impl)
macro_rules! Depcrate_vectorimpl_525 {
() => {
// Module: crate::vector
// Provides: {"impl_525"}
// Dependencies: {}
impl < A : Clone + PartialOrd > PartialOrd for Vector < A > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . iter () . partial_cmp (other . iter ()) } }
};
}
