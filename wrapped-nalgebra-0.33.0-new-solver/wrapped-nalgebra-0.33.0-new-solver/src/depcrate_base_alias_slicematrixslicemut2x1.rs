// Generated macro for MatrixSliceMut2x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2x1"}
// Dependencies: {}
# [doc = " A column-major 2x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut2x1)] pub type MatrixSliceMut2x1 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorageMut < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
