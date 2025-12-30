// Generated macro for impl_778 (impl)
macro_rules! Depcrate_base_conversionimpl_778 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_778"}
// Dependencies: {}
impl < 'a , T : Scalar , RStride : Dim , CStride : Dim , const R : usize , const C : usize > From < MatrixViewMut < 'a , T , Const < R > , Const < C > , RStride , CStride > > for [[T ; R] ; C] { # [inline] fn from (mat : MatrixViewMut < 'a , T , Const < R > , Const < C > , RStride , CStride >) -> Self { mat . into_owned () . into () } }
};
}
