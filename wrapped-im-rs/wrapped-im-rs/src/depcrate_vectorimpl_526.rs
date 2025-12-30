// Generated macro for impl_526 (impl)
macro_rules! Depcrate_vectorimpl_526 {
() => {
// Module: crate::vector
// Provides: {"impl_526"}
// Dependencies: {}
impl < A : Clone + Ord > Ord for Vector < A > { fn cmp (& self , other : & Self) -> Ordering { self . iter () . cmp (other . iter ()) } }
};
}
