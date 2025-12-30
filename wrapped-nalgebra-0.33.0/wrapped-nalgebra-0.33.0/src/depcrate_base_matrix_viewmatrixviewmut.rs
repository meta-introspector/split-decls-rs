// Generated macro for MatrixViewMut (type)
macro_rules! Depcrate_base_matrix_viewMatrixViewMut {
() => {
// Module: crate::base::matrix_view
// Provides: {"MatrixViewMut"}
// Dependencies: {}
# [doc = " A mutable matrix view."] pub type MatrixViewMut < 'a , T , R , C , RStride = U1 , CStride = R > = Matrix < T , R , C , ViewStorageMut < 'a , T , R , C , RStride , CStride > > ;
};
}
