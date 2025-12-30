// Generated macro for MatrixSliceMut6x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut6x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut6x1"}
// Dependencies: {}
# [doc = " A column-major 6x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut6x1)] pub type MatrixSliceMut6x1 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorageMut < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
