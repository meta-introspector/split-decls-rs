// Generated macro for MatrixSlice1x2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice1x2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice1x2"}
// Dependencies: {}
# [doc = " A column-major 1x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView1x2)] pub type MatrixSlice1x2 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U2 , ViewStorage < 'a , T , U1 , U2 , RStride , CStride > > ;
};
}
