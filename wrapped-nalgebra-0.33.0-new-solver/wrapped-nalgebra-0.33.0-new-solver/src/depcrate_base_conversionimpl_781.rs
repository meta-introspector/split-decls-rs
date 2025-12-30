// Generated macro for impl_781 (impl)
macro_rules! Depcrate_base_conversionimpl_781 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_781"}
// Dependencies: {}
impl < 'a , T , RStride , CStride , const R : usize , const C : usize > From < MatrixView < 'a , T , Const < R > , Const < C > , RStride , CStride > > for Matrix < T , Const < R > , Const < C > , ArrayStorage < T , R , C > > where T : Scalar , RStride : Dim , CStride : Dim , { fn from (matrix_view : MatrixView < 'a , T , Const < R > , Const < C > , RStride , CStride >) -> Self { matrix_view . into_owned () } }
};
}
