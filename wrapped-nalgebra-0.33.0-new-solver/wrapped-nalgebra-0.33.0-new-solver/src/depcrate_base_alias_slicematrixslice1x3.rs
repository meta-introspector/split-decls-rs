// Generated macro for MatrixSlice1x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice1x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice1x3"}
// Dependencies: {}
# [doc = " A column-major 1x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView1x3)] pub type MatrixSlice1x3 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U3 , ViewStorage < 'a , T , U1 , U3 , RStride , CStride > > ;
};
}
