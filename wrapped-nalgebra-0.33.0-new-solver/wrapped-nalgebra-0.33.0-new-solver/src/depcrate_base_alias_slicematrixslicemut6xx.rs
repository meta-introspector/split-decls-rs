// Generated macro for MatrixSliceMut6xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut6xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut6xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 6 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut6xX)] pub type MatrixSliceMut6xX < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , Dyn , ViewStorageMut < 'a , T , U6 , Dyn , RStride , CStride > > ;
};
}
