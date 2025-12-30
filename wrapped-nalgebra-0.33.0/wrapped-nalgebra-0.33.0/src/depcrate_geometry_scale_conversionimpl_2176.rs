// Generated macro for impl_2176 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2176 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2176"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < [T ; D] > for Scale < T , D > { # [inline] fn from (coords : [T ; D]) -> Self { Scale { vector : coords . into () , } } }
};
}
