// Generated macro for MatrixSum (type)
macro_rules! Depcrate_base_matrixMatrixSum {
() => {
// Module: crate::base::matrix
// Provides: {"MatrixSum"}
// Dependencies: {}
# [doc = " The type of the result of a matrix sum."] pub type MatrixSum < T , R1 , C1 , R2 , C2 > = Matrix < T , SameShapeR < R1 , R2 > , SameShapeC < C1 , C2 > , SameShapeStorage < T , R1 , C1 , R2 , C2 > > ;
};
}
