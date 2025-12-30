// Generated macro for MatrixSlice2x1 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice2x1 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice2x1"}
// Dependencies: {}
# [doc = " A column-major 2x1 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView2x1)] pub type MatrixSlice2x1 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorage < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
