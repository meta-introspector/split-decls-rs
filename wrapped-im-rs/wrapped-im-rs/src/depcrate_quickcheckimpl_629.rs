// Generated macro for impl_629 (impl)
macro_rules! Depcrate_quickcheckimpl_629 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_629"}
// Dependencies: {}
impl < K : Ord + Clone + Arbitrary + Sync , V : Clone + Arbitrary + Sync > Arbitrary for OrdMap < K , V > { fn arbitrary (g : & mut Gen) -> Self { OrdMap :: from_iter (Vec :: < (K , V) > :: arbitrary (g)) } }
};
}
