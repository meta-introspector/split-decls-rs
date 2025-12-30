// Generated macro for impl_2061 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2061 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2061"}
// Dependencies: {}
impl < T : Scalar , const D : usize > From < OVector < T , Const < D > > > for Translation < T , D > { # [inline] fn from (vector : OVector < T , Const < D > >) -> Self { Translation { vector } } }
};
}
