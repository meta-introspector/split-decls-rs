// Generated macro for MatrixSliceMut1x5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut1x5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut1x5"}
// Dependencies: {}
# [doc = " A column-major 1x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut1x5)] pub type MatrixSliceMut1x5 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U5 , ViewStorageMut < 'a , T , U1 , U5 , RStride , CStride > > ;
};
}
