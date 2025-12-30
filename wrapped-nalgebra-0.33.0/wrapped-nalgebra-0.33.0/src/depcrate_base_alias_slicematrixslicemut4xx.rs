// Generated macro for MatrixSliceMut4xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut4xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut4xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 4 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut4xX)] pub type MatrixSliceMut4xX < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , Dyn , ViewStorageMut < 'a , T , U4 , Dyn , RStride , CStride > > ;
};
}
