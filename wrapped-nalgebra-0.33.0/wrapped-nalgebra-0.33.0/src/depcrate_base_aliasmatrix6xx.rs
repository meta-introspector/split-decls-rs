// Generated macro for Matrix6xX (type)
macro_rules! Depcrate_base_aliasMatrix6xX {
() => {
// Module: crate::base::alias
// Provides: {"Matrix6xX"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with 6 rows and a dynamic number of columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type Matrix6xX < T > = Matrix < T , U6 , Dyn , VecStorage < T , U6 , Dyn > > ;
};
}
