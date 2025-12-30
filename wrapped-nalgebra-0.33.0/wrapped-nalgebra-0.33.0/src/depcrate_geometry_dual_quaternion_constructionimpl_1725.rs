// Generated macro for impl_1725 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_constructionimpl_1725 {
() => {
// Module: crate::geometry::dual_quaternion_construction
// Provides: {"impl_1725"}
// Dependencies: {}
impl < T : SimdRealField > DualQuaternion < T > where T :: Element : SimdRealField , { # [doc = " Creates a dual quaternion from only its real part, with no translation"] # [doc = " component."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{DualQuaternion, Quaternion};"] # [doc = " let rot = Quaternion::new(1.0, 2.0, 3.0, 4.0);"] # [doc = ""] # [doc = " let dq = DualQuaternion::from_real(rot);"] # [doc = " assert_eq!(dq.real.w, 1.0);"] # [doc = " assert_eq!(dq.dual.w, 0.0);"] # [doc = " ```"] # [inline] pub fn from_real (real : Quaternion < T >) -> Self { Self { real , dual : Quaternion :: zero () , } } }
};
}
