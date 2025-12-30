// Generated macro for Matrix1xX (type)
macro_rules! Depcrate_base_aliasMatrix1xX {
() => {
// Module: crate::base::alias
// Provides: {"Matrix1xX"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with 1 rows and a dynamic number of columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type Matrix1xX < T > = Matrix < T , U1 , Dyn , VecStorage < T , U1 , Dyn > > ;
};
}
