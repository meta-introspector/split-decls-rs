// Generated macro for impl_2062 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2062 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2062"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < [T ; D] > for Translation < T , D > { # [inline] fn from (coords : [T ; D]) -> Self { Translation { vector : coords . into () , } } }
};
}
