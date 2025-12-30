// Generated macro for impl_1610 (impl)
macro_rules! Depcrate_geometry_quaternion_opsimpl_1610 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"impl_1610"}
// Dependencies: {}
impl < T : Scalar > IndexMut < usize > for Quaternion < T > { # [inline] fn index_mut (& mut self , i : usize) -> & mut T { & mut self . coords [i] } }
};
}
