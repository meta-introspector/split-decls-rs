// Generated macro for impl_786 (impl)
macro_rules! Depcrate_base_conversionimpl_786 {
() => {
// Module: crate::base::conversion
// Provides: {"impl_786"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , T , R , RStride , CStride > From < MatrixViewMut < 'a , T , R , Dyn , RStride , CStride > > for Matrix < T , R , Dyn , VecStorage < T , R , Dyn > > where T : Scalar , R : DimName , RStride : Dim , CStride : Dim , { fn from (matrix_view : MatrixViewMut < 'a , T , R , Dyn , RStride , CStride >) -> Self { matrix_view . into_owned () } }
};
}
