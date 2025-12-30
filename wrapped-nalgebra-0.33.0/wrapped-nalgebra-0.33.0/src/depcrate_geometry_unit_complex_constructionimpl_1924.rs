// Generated macro for impl_1924 (impl)
macro_rules! Depcrate_geometry_unit_complex_constructionimpl_1924 {
() => {
// Module: crate::geometry::unit_complex_construction
// Provides: {"impl_1924"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : SimdRealField + Arbitrary > Arbitrary for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn arbitrary (g : & mut Gen) -> Self { UnitComplex :: from_angle (T :: arbitrary (g)) } }
};
}
