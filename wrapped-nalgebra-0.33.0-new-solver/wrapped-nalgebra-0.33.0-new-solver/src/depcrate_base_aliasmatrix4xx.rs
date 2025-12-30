// Generated macro for Matrix4xX (type)
macro_rules! Depcrate_base_aliasMatrix4xX {
() => {
// Module: crate::base::alias
// Provides: {"Matrix4xX"}
// Dependencies: {}
# [doc = " A heap-allocated, column-major, matrix with 4 rows and a dynamic number of columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [cfg (any (feature = "std" , feature = "alloc"))] pub type Matrix4xX < T > = Matrix < T , U4 , Dyn , VecStorage < T , U4 , Dyn > > ;
};
}
