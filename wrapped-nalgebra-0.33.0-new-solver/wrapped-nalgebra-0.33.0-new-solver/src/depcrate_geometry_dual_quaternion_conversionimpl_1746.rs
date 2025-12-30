// Generated macro for impl_1746 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_conversionimpl_1746 {
() => {
// Module: crate::geometry::dual_quaternion_conversion
// Provides: {"impl_1746"}
// Dependencies: {}
impl < T : SimdRealField > From < Isometry3 < T > > for UnitDualQuaternion < T > where T :: Element : SimdRealField , { # [inline] fn from (iso : Isometry3 < T >) -> Self { Self :: from_isometry (& iso) } }
};
}
