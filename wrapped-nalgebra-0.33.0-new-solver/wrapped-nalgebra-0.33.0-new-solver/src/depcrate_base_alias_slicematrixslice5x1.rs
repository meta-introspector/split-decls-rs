// Generated macro for MatrixSlice5x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice5x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice5x1"}
// Dependencies: {}
# [doc = " A column-major 5x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView5x1)] pub type MatrixSlice5x1 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorage < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
