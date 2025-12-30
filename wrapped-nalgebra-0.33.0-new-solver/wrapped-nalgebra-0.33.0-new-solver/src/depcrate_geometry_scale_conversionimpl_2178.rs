// Generated macro for impl_2178 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2178 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2178"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < Scale < T , D > > for [T ; D] { # [inline] fn from (t : Scale < T , D >) -> Self { t . vector . into () } }
};
}
