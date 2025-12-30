// Generated macro for impl_630 (impl)
macro_rules! Depcrate_quickcheckimpl_630 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_630"}
// Dependencies: {}
impl < A : Ord + Clone + Arbitrary + Sync > Arbitrary for OrdSet < A > { fn arbitrary (g : & mut Gen) -> Self { OrdSet :: from_iter (Vec :: < A > :: arbitrary (g)) } }
};
}
