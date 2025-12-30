// Generated macro for impl_1319 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1319 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1319"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < [T ; D] > for Point < T , D > { # [inline] fn from (coords : [T ; D]) -> Self { Point { coords : coords . into () , } } }
};
}
