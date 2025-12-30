// Generated macro for MatrixSliceMut2x5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut2x5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut2x5"}
// Dependencies: {}
# [doc = " A column-major 2x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut2x5)] pub type MatrixSliceMut2x5 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U5 , ViewStorageMut < 'a , T , U2 , U5 , RStride , CStride > > ;
};
}
