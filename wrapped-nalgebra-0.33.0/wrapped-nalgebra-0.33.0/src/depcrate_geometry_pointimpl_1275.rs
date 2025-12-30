// Generated macro for impl_1275 (impl)
macro_rules! Depcrate_geometry_pointimpl_1275 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1275"}
// Dependencies: {}
impl < T : Scalar , D : DimName > PartialEq for OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [inline] fn eq (& self , right : & Self) -> bool { self . coords == right . coords } }
};
}
