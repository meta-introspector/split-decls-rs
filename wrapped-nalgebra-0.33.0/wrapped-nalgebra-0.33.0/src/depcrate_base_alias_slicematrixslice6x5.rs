// Generated macro for MatrixSlice6x5 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice6x5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice6x5"}
// Dependencies: {}
# [doc = " A column-major 6x5 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView6x5)] pub type MatrixSlice6x5 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U5 , ViewStorage < 'a , T , U6 , U5 , RStride , CStride > > ;
};
}
