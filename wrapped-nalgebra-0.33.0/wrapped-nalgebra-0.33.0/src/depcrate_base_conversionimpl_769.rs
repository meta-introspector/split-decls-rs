// Generated macro for impl_769 (impl)
macro_rules! Depcrate_base_conversionimpl_769 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_769"}
// Dependencies: {}
impl < 'a , T : Scalar , RStride : Dim , CStride : Dim , const D : usize > From < VectorView < 'a , T , Const < D > , RStride , CStride > > for [T ; D] { # [inline] fn from (vec : VectorView < 'a , T , Const < D > , RStride , CStride >) -> Self { vec . into_owned () . into () } }
};
}
