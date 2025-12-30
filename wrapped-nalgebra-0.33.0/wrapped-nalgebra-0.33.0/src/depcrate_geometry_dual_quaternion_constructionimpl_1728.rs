// Generated macro for impl_1728 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_constructionimpl_1728 {
() => {
// Module: crate::geometry::dual_quaternion_construction
// Provides: {"impl_1728"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T > Arbitrary for DualQuaternion < T > where T : SimdRealField + Arbitrary + Send , T :: Element : SimdRealField , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { Self :: from_real_and_dual (Arbitrary :: arbitrary (rng) , Arbitrary :: arbitrary (rng)) } }
};
}
