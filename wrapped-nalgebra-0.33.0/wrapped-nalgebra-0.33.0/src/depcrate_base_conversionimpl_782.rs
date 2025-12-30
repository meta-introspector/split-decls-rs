// Generated macro for impl_782 (impl)
macro_rules! Depcrate_base_conversionimpl_782 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_782"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , T , C , RStride , CStride > From < MatrixView < 'a , T , Dyn , C , RStride , CStride > > for Matrix < T , Dyn , C , VecStorage < T , Dyn , C > > where T : Scalar , C : Dim , RStride : Dim , CStride : Dim , { fn from (matrix_view : MatrixView < 'a , T , Dyn , C , RStride , CStride >) -> Self { matrix_view . into_owned () } }
};
}
