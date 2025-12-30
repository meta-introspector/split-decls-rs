// Generated macro for MatrixSliceMut5x6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut5x6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut5x6"}
// Dependencies: {}
# [doc = " A column-major 5x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut5x6)] pub type MatrixSliceMut5x6 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U6 , ViewStorageMut < 'a , T , U5 , U6 , RStride , CStride > > ;
};
}
