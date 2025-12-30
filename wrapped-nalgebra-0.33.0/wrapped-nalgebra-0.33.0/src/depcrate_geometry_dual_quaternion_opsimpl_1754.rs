// Generated macro for impl_1754 (impl)
macro_rules! Depcrate_geometry_dual_quaternion_opsimpl_1754 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"impl_1754"}
// Dependencies: {}
impl < T : SimdRealField > IndexMut < usize > for DualQuaternion < T > { # [inline] fn index_mut (& mut self , i : usize) -> & mut T { & mut self . as_mut () [i] } }
};
}
