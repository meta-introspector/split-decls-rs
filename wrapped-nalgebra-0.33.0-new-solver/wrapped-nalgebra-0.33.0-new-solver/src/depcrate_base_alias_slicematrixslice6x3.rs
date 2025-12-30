// Generated macro for MatrixSlice6x3 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice6x3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice6x3"}
// Dependencies: {}
# [doc = " A column-major 6x3 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView6x3)] pub type MatrixSlice6x3 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U3 , ViewStorage < 'a , T , U6 , U3 , RStride , CStride > > ;
};
}
