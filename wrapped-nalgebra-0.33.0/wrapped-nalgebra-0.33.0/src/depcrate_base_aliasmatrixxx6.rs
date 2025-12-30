// Generated macro for MatrixXx6 (type)
macro_rules! Depcrate_base_aliasMatrixXx6 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx6"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 6 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx6 < T > = Matrix < T , Dyn , U6 , VecStorage < T , Dyn , U6 > > ;
};
}
