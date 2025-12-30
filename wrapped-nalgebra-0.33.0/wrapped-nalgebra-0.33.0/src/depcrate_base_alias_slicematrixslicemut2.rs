// Generated macro for MatrixSliceMut2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2"}
// Dependencies: {}
# [doc = " A column-major 2x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut2)] pub type MatrixSliceMut2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U2 , ViewStorageMut < 'a , T , U2 , U2 , RStride , CStride > > ;
};
}
