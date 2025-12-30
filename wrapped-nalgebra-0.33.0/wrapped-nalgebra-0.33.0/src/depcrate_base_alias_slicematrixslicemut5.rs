// Generated macro for MatrixSliceMut5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut5"}
// Dependencies: {}
# [doc = " A column-major 5x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut5)] pub type MatrixSliceMut5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U5 , ViewStorageMut < 'a , T , U5 , U5 , RStride , CStride > > ;
};
}
