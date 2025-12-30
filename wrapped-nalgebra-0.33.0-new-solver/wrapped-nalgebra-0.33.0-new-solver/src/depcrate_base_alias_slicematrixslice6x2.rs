// Generated macro for MatrixSlice6x2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice6x2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice6x2"}
// Dependencies: {}
# [doc = " A column-major 6x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView6x2)] pub type MatrixSlice6x2 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U2 , ViewStorage < 'a , T , U6 , U2 , RStride , CStride > > ;
};
}
