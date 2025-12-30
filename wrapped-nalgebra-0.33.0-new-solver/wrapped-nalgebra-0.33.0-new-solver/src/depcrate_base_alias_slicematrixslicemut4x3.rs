// Generated macro for MatrixSliceMut4x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut4x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut4x3"}
// Dependencies: {}
# [doc = " A column-major 4x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut4x3)] pub type MatrixSliceMut4x3 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U3 , ViewStorageMut < 'a , T , U4 , U3 , RStride , CStride > > ;
};
}
