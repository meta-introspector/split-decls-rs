// Generated macro for MatrixXx3 (type)
macro_rules! Depcrate_base_aliasMatrixXx3 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx3"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 3 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx3 < T > = Matrix < T , Dyn , U3 , VecStorage < T , Dyn , U3 > > ;
};
}
