// Generated macro for MatrixSlice (type)
macro_rules! Depcrate_base_matrix_viewMatrixSlice {
() => {
// Module: crate::base::matrix_view
// Provides: {"MatrixSlice"}
// Dependencies: {}
# [doc = " A matrix slice."] # [doc = ""] # [doc = " This type alias exists only for legacy purposes and is deprecated. It will be removed"] # [doc = " in a future release. Please use [`MatrixView`] instead."] # [doc = " See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076)"] # [doc = " for the rationale."] # [deprecated = "Use MatrixView instead."] pub type MatrixSlice < 'a , T , R , C , RStride = U1 , CStride = R > = MatrixView < 'a , T , R , C , RStride , CStride > ;
};
}
