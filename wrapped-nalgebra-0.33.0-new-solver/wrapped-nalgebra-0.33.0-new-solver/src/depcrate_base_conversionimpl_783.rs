// Generated macro for impl_783 (impl)
macro_rules! Depcrate_base_conversionimpl_783 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_783"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , T , R , RStride , CStride > From < MatrixView < 'a , T , R , Dyn , RStride , CStride > > for Matrix < T , R , Dyn , VecStorage < T , R , Dyn > > where T : Scalar , R : DimName , RStride : Dim , CStride : Dim , { fn from (matrix_view : MatrixView < 'a , T , R , Dyn , RStride , CStride >) -> Self { matrix_view . into_owned () } }
};
}
