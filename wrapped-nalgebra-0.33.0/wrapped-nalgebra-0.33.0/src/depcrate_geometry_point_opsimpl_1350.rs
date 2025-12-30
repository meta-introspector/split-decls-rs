// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_geometry_point_opsimpl_1350 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"impl_1350"}
// Dependencies: {}
impl < T : Scalar , D : DimName > IndexMut < usize > for OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [inline] fn index_mut (& mut self , i : usize) -> & mut Self :: Output { & mut self . coords [i] } }
};
}
