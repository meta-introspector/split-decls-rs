// Generated macro for MatrixSliceMut5x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut5x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut5x3"}
// Dependencies: {}
# [doc = " A column-major 5x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut5x3)] pub type MatrixSliceMut5x3 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U3 , ViewStorageMut < 'a , T , U5 , U3 , RStride , CStride > > ;
};
}
