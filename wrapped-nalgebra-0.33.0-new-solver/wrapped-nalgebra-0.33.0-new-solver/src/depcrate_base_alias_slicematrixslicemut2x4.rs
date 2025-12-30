// Generated macro for MatrixSliceMut2x4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2x4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2x4"}
// Dependencies: {}
# [doc = " A column-major 2x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut2x4)] pub type MatrixSliceMut2x4 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U4 , ViewStorageMut < 'a , T , U2 , U4 , RStride , CStride > > ;
};
}
