// Generated macro for MatrixSliceMut (type)
macro_rules! Depcrate_base_matrix_viewMatrixSliceMut {
() => {
// Module: crate::base::matrix_view
// Provides: {"MatrixSliceMut"}
// Dependencies: {}
# [doc = " A mutable matrix slice."] # [doc = ""] # [doc = " This type alias exists only for legacy purposes and is deprecated. It will be removed"] # [doc = " in a future release. Please use [`MatrixViewMut`] instead."] # [doc = " See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076)"] # [doc = " for the rationale."] # [deprecated = "Use MatrixViewMut instead."] pub type MatrixSliceMut < 'a , T , R , C , RStride = U1 , CStride = R > = MatrixViewMut < 'a , T , R , C , RStride , CStride > ;
};
}
