// Generated macro for MatrixSliceMut3xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut3xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut3xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 3 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut3xX)] pub type MatrixSliceMut3xX < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , Dyn , ViewStorageMut < 'a , T , U3 , Dyn , RStride , CStride > > ;
};
}
