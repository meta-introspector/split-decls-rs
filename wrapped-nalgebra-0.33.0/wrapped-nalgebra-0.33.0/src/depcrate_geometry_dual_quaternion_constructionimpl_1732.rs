// Generated macro for impl_1732 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_constructionimpl_1732 {
() => {
// Module: crate::geometry::dual_quaternion_construction
// Provides: {"impl_1732"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T > Arbitrary for UnitDualQuaternion < T > where T : SimdRealField + Arbitrary + Send , T :: Element : SimdRealField , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { Self :: new_normalize (Arbitrary :: arbitrary (rng)) } }
};
}
