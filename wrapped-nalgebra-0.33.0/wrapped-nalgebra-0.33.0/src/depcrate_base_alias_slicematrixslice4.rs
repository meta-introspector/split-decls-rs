// Generated macro for MatrixSlice4 (type)
macro_rules! Depcrate_base_alias_sliceMatrixSlice4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"MatrixSlice4"}
// Dependencies: {}
# [doc = " A column-major 4x4 matrix slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (MatrixView4)] pub type MatrixSlice4 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U4 , ViewStorage < 'a , T , U4 , U4 , RStride , CStride > > ;
};
}
