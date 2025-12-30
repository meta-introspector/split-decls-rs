// Generated macro for MatrixXx5 (type)
macro_rules! Depcrate_base_aliasMatrixXx5 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx5"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 5 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx5 < T > = Matrix < T , Dyn , U5 , VecStorage < T , Dyn , U5 > > ;
};
}
