// Generated macro for impl_770 (impl)
macro_rules! Depcrate_base_conversionimpl_770 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_770"}
// Dependencies: {}
impl < 'a , T : Scalar , RStride : Dim , CStride : Dim , const D : usize > From < VectorViewMut < 'a , T , Const < D > , RStride , CStride > > for [T ; D] { # [inline] fn from (vec : VectorViewMut < 'a , T , Const < D > , RStride , CStride >) -> Self { vec . into_owned () . into () } }
};
}
