// Generated macro for impl_1320 (impl)
macro_rules! Depcrate_geometry_point_conversionimpl_1320 {
() => {
// Module: crate::geometry::point_conversion
// Provides: {"impl_1320"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < Point < T , D > > for [T ; D] { # [inline] fn from (p : Point < T , D >) -> Self { p . coords . into () } }
};
}
