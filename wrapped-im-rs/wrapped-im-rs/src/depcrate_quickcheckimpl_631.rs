// Generated macro for impl_631 (impl)
macro_rules! Depcrate_quickcheckimpl_631 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_631"}
// Dependencies: {}
impl < A , S > Arbitrary for HashSet < A , S > where A : Hash + Eq + Arbitrary + Sync , S : BuildHasher + Default + Send + Sync + 'static , { fn arbitrary (g : & mut Gen) -> Self { HashSet :: from_iter (Vec :: < A > :: arbitrary (g)) } }
};
}
