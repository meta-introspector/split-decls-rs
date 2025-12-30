// Generated macro for MatrixSlice5x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice5x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice5x3"}
// Dependencies: {}
# [doc = " A column-major 5x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView5x3)] pub type MatrixSlice5x3 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U3 , ViewStorage < 'a , T , U5 , U3 , RStride , CStride > > ;
};
}
