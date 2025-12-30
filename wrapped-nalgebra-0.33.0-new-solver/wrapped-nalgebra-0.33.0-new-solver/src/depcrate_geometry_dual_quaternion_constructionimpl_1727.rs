// Generated macro for impl_1727 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_constructionimpl_1727 {
() => {
// Module: crate::geometry::dual_quaternion_construction
// Provides: {"impl_1727"}
// Dependencies: {}
impl < T : SimdRealField > Zero for DualQuaternion < T > where T :: Element : SimdRealField , { # [inline] fn zero () -> Self { DualQuaternion :: from_real_and_dual (Quaternion :: zero () , Quaternion :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . real . is_zero () && self . dual . is_zero () } }
};
}
