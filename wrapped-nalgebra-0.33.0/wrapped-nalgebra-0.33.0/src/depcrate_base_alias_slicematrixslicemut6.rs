// Generated macro for MatrixSliceMut6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut6"}
// Dependencies: {}
# [doc = " A column-major 6x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut6)] pub type MatrixSliceMut6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U6 , ViewStorageMut < 'a , T , U6 , U6 , RStride , CStride > > ;
};
}
