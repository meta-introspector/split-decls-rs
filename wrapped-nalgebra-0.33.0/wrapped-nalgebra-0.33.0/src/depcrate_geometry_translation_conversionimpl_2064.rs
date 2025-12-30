// Generated macro for impl_2064 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2064 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2064"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < Translation < T , D > > for [T ; D] { # [inline] fn from (t : Translation < T , D >) -> Self { t . vector . into () } }
};
}
