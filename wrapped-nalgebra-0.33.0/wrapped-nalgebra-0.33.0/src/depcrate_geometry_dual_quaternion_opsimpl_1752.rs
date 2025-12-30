// Generated macro for impl_1752 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1752 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1752"}
// Dependencies: {}
impl < T : SimdRealField > AsMut < [T ; 8] > for DualQuaternion < T > { # [inline] fn as_mut (& mut self) -> & mut [T ; 8] { unsafe { & mut * (self as * mut Self as * mut [T ; 8]) } } }
};
}
