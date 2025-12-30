// Generated macro for impl_2063 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2063 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2063"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < Point < T , D > > for Translation < T , D > { # [inline] fn from (pt : Point < T , D >) -> Self { Translation { vector : pt . coords } } }
};
}
