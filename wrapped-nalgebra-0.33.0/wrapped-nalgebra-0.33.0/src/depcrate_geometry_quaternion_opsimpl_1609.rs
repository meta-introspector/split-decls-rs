// Generated macro for impl_1609 (impl)
macro_rules! Depcrate_geometry_quaternion_opsimpl_1609 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"impl_1609"}
// Dependencies: {}
impl < T : Scalar > Index < usize > for Quaternion < T > { type Output = T ; # [inline] fn index (& self , i : usize) -> & Self :: Output { & self . coords [i] } }
};
}
