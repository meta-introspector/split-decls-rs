// Generated macro for MatrixSliceMut4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSliceMut4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSliceMut4"}
// Dependencies: {}
# [doc = " A column-major 4x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixViewMut4)] pub type MatrixSliceMut4 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U4 , ViewStorageMut < 'a , T , U4 , U4 , RStride , CStride > > ;
};
}
