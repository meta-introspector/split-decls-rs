// Generated macro for impl_632 (impl)
macro_rules! Depcrate_quickcheckimpl_632 {
() => {
// Module: crate::quickcheck
// Provides: {"impl_632"}
// Dependencies: {}
impl < K , V , S > Arbitrary for HashMap < K , V , S > where K : Hash + Eq + Arbitrary + Sync , V : Arbitrary + Sync , S : BuildHasher + Default + Send + Sync + 'static , { fn arbitrary (g : & mut Gen) -> Self { HashMap :: from (Vec :: < (K , V) > :: arbitrary (g)) } }
};
}
