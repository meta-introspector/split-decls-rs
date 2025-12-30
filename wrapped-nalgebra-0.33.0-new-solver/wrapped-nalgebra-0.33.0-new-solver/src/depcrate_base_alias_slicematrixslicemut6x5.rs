// Generated macro for MatrixSliceMut6x5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut6x5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut6x5"}
// Dependencies: {}
# [doc = " A column-major 6x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut6x5)] pub type MatrixSliceMut6x5 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U5 , ViewStorageMut < 'a , T , U6 , U5 , RStride , CStride > > ;
};
}
