// Generated macro for impl_1745 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1745 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1745"}
// Dependencies: {}
impl < T : SimdRealField > From < UnitDualQuaternion < T > > for Isometry3 < T > where T :: Element : SimdRealField , { # [inline] fn from (dq : UnitDualQuaternion < T >) -> Self { dq . to_isometry () } }
};
}
