// Generated macro for impl_2177 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2177 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2177"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < Point < T , D > > for Scale < T , D > { # [inline] fn from (pt : Point < T , D >) -> Self { Scale { vector : pt . coords } } }
};
}
