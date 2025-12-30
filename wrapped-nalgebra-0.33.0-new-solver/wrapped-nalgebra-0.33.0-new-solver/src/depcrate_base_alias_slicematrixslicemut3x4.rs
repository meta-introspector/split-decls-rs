// Generated macro for MatrixSliceMut3x4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut3x4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut3x4"}
// Dependencies: {}
# [doc = " A column-major 3x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut3x4)] pub type MatrixSliceMut3x4 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U4 , ViewStorageMut < 'a , T , U3 , U4 , RStride , CStride > > ;
};
}
