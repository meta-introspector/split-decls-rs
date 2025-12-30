// Generated macro for MatrixSliceMut2x6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2x6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2x6"}
// Dependencies: {}
# [doc = " A column-major 2x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut2x6)] pub type MatrixSliceMut2x6 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U6 , ViewStorageMut < 'a , T , U2 , U6 , RStride , CStride > > ;
};
}
