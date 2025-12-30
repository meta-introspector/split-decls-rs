// Generated macro for MatrixXx2 (type)
macro_rules! Depcrate_base_aliasMatrixXx2 {
() => {
// Module: crate::base::alias
// Provides: {"MatrixXx2"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with a dynamic number of rows and 2 columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type MatrixXx2 < T > = Matrix < T , Dyn , U2 , VecStorage < T , Dyn , U2 > > ;
};
}
