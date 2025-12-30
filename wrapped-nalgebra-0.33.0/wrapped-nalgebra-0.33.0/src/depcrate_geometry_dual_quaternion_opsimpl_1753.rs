// Generated macro for impl_1753 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1753 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1753"}
// Dependencies: {}
impl < T : SimdRealField > Index < usize > for DualQuaternion < T > { type Output = T ; # [inline] fn index (& self , i : usize) -> & Self :: Output { & self . as_ref () [i] } }
};
}
