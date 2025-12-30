// Generated macro for impl_1349 (impl)
macro_rules! Depcrate_geometry_point_opsimpl_1349 {
() => {
// Module: crate::geometry::point_ops
// Provides: {"impl_1349"}
// Dependencies: {}
impl < T : Scalar , D : DimName > Index < usize > for OPoint < T , D > where DefaultAllocator : Allocator < D > , { type Output = T ; # [inline] fn index (& self , i : usize) -> & Self :: Output { & self . coords [i] } }
};
}
