// Generated macro for impl_1461 (impl)
macro_rules! Depcrate_geometry_rotation_opsimpl_1461 {
() => {
// Module: crate::geometry::rotation_ops
// Provides: {"impl_1461"}
// Dependencies: {}
impl < T : Scalar , const D : usize > Index < (usize , usize) > for Rotation < T , D > { type Output = T ; # [inline] fn index (& self , row_col : (usize , usize)) -> & T { self . matrix () . index (row_col) } }
};
}
