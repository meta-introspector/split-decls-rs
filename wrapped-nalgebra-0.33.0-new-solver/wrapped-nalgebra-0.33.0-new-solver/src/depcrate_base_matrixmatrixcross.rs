// Generated macro for MatrixCross (type)
macro_rules! Depcrate_base_matrixMatrixCross {
() => {
// Module: crate::base::matrix
// Provides: {"MatrixCross"}
// Dependencies: {}
# [doc = " The type of the result of a matrix cross product."] pub type MatrixCross < T , R1 , C1 , R2 , C2 > = Matrix < T , SameShapeR < R1 , R2 > , SameShapeC < C1 , C2 > , SameShapeStorage < T , R1 , C1 , R2 , C2 > > ;
};
}
