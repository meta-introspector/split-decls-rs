// Generated macro for MatrixSliceMut5x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut5x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut5x1"}
// Dependencies: {}
# [doc = " A column-major 5x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut5x1)] pub type MatrixSliceMut5x1 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorageMut < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
