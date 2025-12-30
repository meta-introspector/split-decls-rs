// Generated macro for Matrix3 (type)
macro_rules! Depcrate_base_aliasMatrix3 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix3"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 3x3 square matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix3 < T > = Matrix < T , U3 , U3 , ArrayStorage < T , 3 , 3 > > ;
};
}
