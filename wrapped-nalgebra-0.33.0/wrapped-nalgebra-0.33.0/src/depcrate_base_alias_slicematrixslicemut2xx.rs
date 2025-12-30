// Generated macro for MatrixSliceMut2xX (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2xX {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2xX"}
// Dependencies: {}
# [doc = " A column-major matrix slice with 2 rows and a number of columns chosen at runtime."] # [deprecated = slice_deprecation_note ! (MatrixViewMut2xX)] pub type MatrixSliceMut2xX < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , Dyn , ViewStorageMut < 'a , T , U2 , Dyn , RStride , CStride > > ;
};
}
