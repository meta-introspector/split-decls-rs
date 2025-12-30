// Generated macro for impl_1756 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1756 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1756"}
// Dependencies: {}
impl < 'a , T : SimdRealField > Neg for & 'a DualQuaternion < T > where T :: Element : SimdRealField , { type Output = DualQuaternion < T > ; # [inline] fn neg (self) -> Self :: Output { DualQuaternion :: from_real_and_dual (- & self . real , - & self . dual) } }
};
}
