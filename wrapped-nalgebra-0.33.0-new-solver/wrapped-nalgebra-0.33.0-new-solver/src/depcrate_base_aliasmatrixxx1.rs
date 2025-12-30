// Generated macro for MatrixXx1 (type)
macro_rules! Depcrate_base_aliasMatrixXx1 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx1"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 1 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx1 < T > = Matrix < T , Dyn , U1 , VecStorage < T , Dyn , U1 > > ;
};
}
