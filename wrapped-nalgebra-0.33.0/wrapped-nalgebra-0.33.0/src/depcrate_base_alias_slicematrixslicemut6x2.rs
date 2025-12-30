// Generated macro for MatrixSliceMut6x2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut6x2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut6x2"}
// Dependencies: {}
# [doc = " A column-major 6x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut6x2)] pub type MatrixSliceMut6x2 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U2 , ViewStorageMut < 'a , T , U6 , U2 , RStride , CStride > > ;
};
}
