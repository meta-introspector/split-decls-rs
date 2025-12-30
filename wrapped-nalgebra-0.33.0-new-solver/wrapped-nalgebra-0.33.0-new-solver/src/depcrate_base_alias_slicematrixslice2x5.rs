// Generated macro for MatrixSlice2x5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice2x5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice2x5"}
// Dependencies: {}
# [doc = " A column-major 2x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView2x5)] pub type MatrixSlice2x5 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U5 , ViewStorage < 'a , T , U2 , U5 , RStride , CStride > > ;
};
}
