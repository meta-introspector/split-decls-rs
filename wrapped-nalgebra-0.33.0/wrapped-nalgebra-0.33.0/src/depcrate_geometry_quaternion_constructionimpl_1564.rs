// Generated macro for impl_1564 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1564 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1564"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : RealField + Arbitrary > Arbitrary for UnitQuaternion < T > where Owned < T , U4 > : Send , Owned < T , U3 > : Send , { # [inline] fn arbitrary (g : & mut Gen) -> Self { let axisangle = Vector3 :: arbitrary (g) ; Self :: from_scaled_axis (axisangle) } }
};
}
