// Generated macro for MatrixSlice2x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice2x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice2x3"}
// Dependencies: {}
# [doc = " A column-major 2x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView2x3)] pub type MatrixSlice2x3 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U3 , ViewStorage < 'a , T , U2 , U3 , RStride , CStride > > ;
};
}
