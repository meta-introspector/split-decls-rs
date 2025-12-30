// Generated macro for impl_1751 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1751 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1751"}
// Dependencies: {}
impl < T : SimdRealField > AsRef < [T ; 8] > for DualQuaternion < T > { # [inline] fn as_ref (& self) -> & [T ; 8] { unsafe { & * (self as * const Self as * const [T ; 8]) } } }
};
}
