// Generated macro for impl_1744 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1744 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1744"}
// Dependencies: {}
impl < T : SimdRealField + RealField > From < UnitDualQuaternion < T > > for Matrix4 < T > where T :: Element : SimdRealField , { # [inline] fn from (dq : UnitDualQuaternion < T >) -> Self { dq . to_homogeneous () } }
};
}
