// Generated macro for Matrix2xX (type)
macro_rules! Depcrate_base_aliasMatrix2xX {
() => {
// Module: crate::base::alias
// Provides: {"Matrix2xX"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with 2 rows and a dynamic number of columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type Matrix2xX < T > = Matrix < T , U2 , Dyn , VecStorage < T , U2 , Dyn > > ;
};
}
