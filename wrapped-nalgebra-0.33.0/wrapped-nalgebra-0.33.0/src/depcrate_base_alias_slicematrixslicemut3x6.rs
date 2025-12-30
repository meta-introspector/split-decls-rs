// Generated macro for MatrixSliceMut3x6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut3x6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut3x6"}
// Dependencies: {}
# [doc = " A column-major 3x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut3x6)] pub type MatrixSliceMut3x6 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U6 , ViewStorageMut < 'a , T , U3 , U6 , RStride , CStride > > ;
};
}
