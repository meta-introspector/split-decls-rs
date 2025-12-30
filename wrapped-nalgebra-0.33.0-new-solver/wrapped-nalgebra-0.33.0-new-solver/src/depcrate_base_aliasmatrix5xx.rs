// Generated macro for Matrix5xX (type)
macro_rules! Depcrate_base_aliasMatrix5xX {
() => {
// Module: crate::base::alias
// Provides: {"Matrix5xX"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with 5 rows and a dynamic number of columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type Matrix5xX < T > = Matrix < T , U5 , Dyn , VecStorage < T , U5 , Dyn > > ;
};
}
