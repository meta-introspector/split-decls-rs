// Generated macro for MatrixSlice5x6 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice5x6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice5x6"}
// Dependencies: {}
# [doc = " A column-major 5x6 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView5x6)] pub type MatrixSlice5x6 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U6 , ViewStorage < 'a , T , U5 , U6 , RStride , CStride > > ;
};
}
