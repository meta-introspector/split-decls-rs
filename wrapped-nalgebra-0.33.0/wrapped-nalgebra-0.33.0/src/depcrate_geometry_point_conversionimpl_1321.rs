// Generated macro for impl_1321 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1321 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1321"}
// Dependencies: {}
impl < T : Scalar , D : DimName > From < OVector < T , D > > for OPoint < T , D > where DefaultAllocator : Allocator < D > , { # [inline] fn from (coords : OVector < T , D >) -> Self { OPoint { coords } } }
};
}
