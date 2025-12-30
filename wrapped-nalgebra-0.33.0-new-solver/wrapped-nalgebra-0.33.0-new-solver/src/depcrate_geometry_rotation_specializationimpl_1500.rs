// Generated macro for impl_1500 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1500 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1500"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : SimdRealField + Arbitrary > Arbitrary for Rotation3 < T > where T :: Element : SimdRealField , Owned < T , U3 , U3 > : Send , Owned < T , U3 > : Send , { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new (SVector :: arbitrary (g)) } }
};
}
