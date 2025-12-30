// Generated macro for impl_1494 (impl)
macro_rules! Depcrate_geometry_rotation_specializationimpl_1494 {
() => {
// Module: crate::geometry::rotation_specialization
// Provides: {"impl_1494"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : SimdRealField + Arbitrary > Arbitrary for Rotation2 < T > where T :: Element : SimdRealField , Owned < T , U2 , U2 > : Send , { # [inline] fn arbitrary (g : & mut Gen) -> Self { Self :: new (T :: arbitrary (g)) } }
};
}
