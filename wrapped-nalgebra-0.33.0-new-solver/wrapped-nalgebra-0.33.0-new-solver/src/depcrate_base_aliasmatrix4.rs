// Generated macro for Matrix4 (type)
macro_rules! Depcrate_base_aliasMatrix4 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix4"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 4x4 square matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix4 < T > = Matrix < T , U4 , U4 , ArrayStorage < T , 4 , 4 > > ;
};
}
