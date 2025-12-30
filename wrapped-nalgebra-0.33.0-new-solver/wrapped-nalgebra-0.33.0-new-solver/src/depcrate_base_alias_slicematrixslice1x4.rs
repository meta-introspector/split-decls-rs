// Generated macro for MatrixSlice1x4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice1x4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice1x4"}
// Dependencies: {}
# [doc = " A column-major 1x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView1x4)] pub type MatrixSlice1x4 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U4 , ViewStorage < 'a , T , U1 , U4 , RStride , CStride > > ;
};
}
