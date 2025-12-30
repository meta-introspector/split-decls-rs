// Generated macro for MatrixSliceMut1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut1"}
// Dependencies: {}
# [doc = " A column-major 1x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut1)] pub type MatrixSliceMut1 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U1 , ViewStorageMut < 'a , T , U1 , U1 , RStride , CStride > > ;
};
}
