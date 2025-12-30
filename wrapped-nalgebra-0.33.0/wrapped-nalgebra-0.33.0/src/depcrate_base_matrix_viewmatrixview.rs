// Generated macro for MatrixView (type)
macro_rules! Depcrate_base_matrix_viewMatrixView {
() => {
// Module: crate::base::matrix_view
// Provides: {"MatrixView"}
// Dependencies: {}
# [doc = " A matrix view."] pub type MatrixView < 'a , T , R , C , RStride = U1 , CStride = R > = Matrix < T , R , C , ViewStorage < 'a , T , R , C , RStride , CStride > > ;
};
}
