// Generated macro for Matrix3x4 (type)
macro_rules! Depcrate_base_aliasMatrix3x4 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix3x4"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 3x4 matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix3x4 < T > = Matrix < T , U3 , U4 , ArrayStorage < T , 3 , 4 > > ;
};
}
