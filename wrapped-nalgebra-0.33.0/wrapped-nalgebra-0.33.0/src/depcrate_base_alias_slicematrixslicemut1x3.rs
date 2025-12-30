// Generated macro for MatrixSliceMut1x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut1x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut1x3"}
// Dependencies: {}
# [doc = " A column-major 1x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut1x3)] pub type MatrixSliceMut1x3 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U3 , ViewStorageMut < 'a , T , U1 , U3 , RStride , CStride > > ;
};
}
