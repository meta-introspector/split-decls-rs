// Generated macro for MatrixSliceMut1xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut1xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut1xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 1 row and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut1xX)] pub type MatrixSliceMut1xX < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , Dyn , ViewStorageMut < 'a , T , U1 , Dyn , RStride , CStride > > ;
};
}
