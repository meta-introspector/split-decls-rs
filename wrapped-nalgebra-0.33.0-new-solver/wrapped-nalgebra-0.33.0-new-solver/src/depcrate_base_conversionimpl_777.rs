// Generated macro for impl_777 (impl)
macro_rules! Depcrate_base_conversionimpl_777 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_777"}
// Dependencies: {}
impl < 'a , T : Scalar , RStride : Dim , CStride : Dim , const R : usize , const C : usize > From < MatrixView < 'a , T , Const < R > , Const < C > , RStride , CStride > > for [[T ; R] ; C] { # [inline] fn from (mat : MatrixView < 'a , T , Const < R > , Const < C > , RStride , CStride >) -> Self { mat . into_owned () . into () } }
};
}
