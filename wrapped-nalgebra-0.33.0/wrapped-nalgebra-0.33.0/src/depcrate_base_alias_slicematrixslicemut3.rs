// Generated macro for MatrixSliceMut3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut3"}
// Dependencies: {}
# [doc = " A column-major 3x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut3)] pub type MatrixSliceMut3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U3 , ViewStorageMut < 'a , T , U3 , U3 , RStride , CStride > > ;
};
}
