// Generated macro for MatrixSlice4x2 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice4x2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice4x2"}
// Dependencies: {}
# [doc = " A column-major 4x2 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView4x2)] pub type MatrixSlice4x2 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U2 , ViewStorage < 'a , T , U4 , U2 , RStride , CStride > > ;
};
}
