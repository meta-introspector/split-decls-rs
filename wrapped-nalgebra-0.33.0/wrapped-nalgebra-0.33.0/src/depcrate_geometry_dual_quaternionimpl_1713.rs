// Generated macro for impl_1713 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1713 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1713"}
// Dependencies: {}
impl < T : SimdRealField + RealField > UnitDualQuaternion < T > where T :: Element : SimdRealField , { # [doc = " Converts this unit dual quaternion interpreted as an isometry"] # [doc = " into its equivalent homogeneous transformation matrix."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # #[macro_use] extern crate approx;"] # [doc = " # use nalgebra::{Matrix4, UnitDualQuaternion, UnitQuaternion, Vector3};"] # [doc = " let dq = UnitDualQuaternion::from_parts("] # [doc = "     Vector3::new(1.0, 3.0, 2.0).into(),"] # [doc = "     UnitQuaternion::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_6)"] # [doc = " );"] # [doc = " let expected = Matrix4::new(0.8660254, -0.5,      0.0, 1.0,"] # [doc = "                             0.5,       0.8660254, 0.0, 3.0,"] # [doc = "                             0.0,       0.0,       1.0, 2.0,"] # [doc = "                             0.0,       0.0,       0.0, 1.0);"] # [doc = ""] # [doc = " assert_relative_eq!(dq.to_homogeneous(), expected, epsilon = 1.0e-6);"] # [doc = " ```"] # [inline] # [must_use] pub fn to_homogeneous (self) -> Matrix4 < T > { self . to_isometry () . to_homogeneous () } }
};
}
