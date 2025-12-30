// Generated macro for MatrixXx4 (type)
macro_rules! Depcrate_base_aliasMatrixXx4 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx4"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 4 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx4 < T > = Matrix < T , Dyn , U4 , VecStorage < T , Dyn , U4 > > ;
};
}
