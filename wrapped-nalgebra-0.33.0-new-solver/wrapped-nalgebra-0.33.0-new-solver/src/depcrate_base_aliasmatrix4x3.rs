// Generated macro for Matrix4x3 (type)
macro_rules! Depcrate_base_aliasMatrix4x3 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix4x3"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 4x3 matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix4x3 < T > = Matrix < T , U4 , U3 , ArrayStorage < T , 4 , 3 > > ;
};
}
