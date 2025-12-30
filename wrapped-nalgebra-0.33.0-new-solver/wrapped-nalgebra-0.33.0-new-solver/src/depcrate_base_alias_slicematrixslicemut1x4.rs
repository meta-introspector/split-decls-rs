// Generated macro for MatrixSliceMut1x4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut1x4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut1x4"}
// Dependencies: {}
# [doc = " A column-major 1x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut1x4)] pub type MatrixSliceMut1x4 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U4 , ViewStorageMut < 'a , T , U1 , U4 , RStride , CStride > > ;
};
}
