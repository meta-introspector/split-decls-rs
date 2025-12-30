// Generated macro for impl_1558 (impl)
macro_rules! Depcrate_geometry_quaternion_constructionimpl_1558 {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"impl_1558"}
// Dependencies: {}
impl < T : SimdRealField > Zero for Quaternion < T > where T :: Element : SimdRealField , { # [inline] fn zero () -> Self { Self :: from (Vector4 :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . coords . is_zero () } }
};
}
