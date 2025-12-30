// Generated macro for impl_628 (impl)
macro_rules! Depcrate_quickcheckimpl_628 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_628"}
// Dependencies: {}
impl < A : Arbitrary + Sync + Clone > Arbitrary for Vector < A > { fn arbitrary (g : & mut Gen) -> Self { Vector :: from_iter (Vec :: < A > :: arbitrary (g)) } }
};
}
