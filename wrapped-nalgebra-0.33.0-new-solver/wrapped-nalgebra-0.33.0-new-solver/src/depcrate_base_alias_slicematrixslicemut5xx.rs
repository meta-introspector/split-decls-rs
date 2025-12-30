// Generated macro for MatrixSliceMut5xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut5xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut5xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 5 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut5xX)] pub type MatrixSliceMut5xX < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , Dyn , ViewStorageMut < 'a , T , U5 , Dyn , RStride , CStride > > ;
};
}
