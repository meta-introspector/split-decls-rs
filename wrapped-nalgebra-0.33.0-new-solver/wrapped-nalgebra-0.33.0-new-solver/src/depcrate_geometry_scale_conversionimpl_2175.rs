// Generated macro for impl_2175 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2175 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2175"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < OVector < T , Const < D > > > for Scale < T , D > { # [inline] fn from (vector : OVector < T , Const < D > >) -> Self { Scale { vector } } }
};
}
