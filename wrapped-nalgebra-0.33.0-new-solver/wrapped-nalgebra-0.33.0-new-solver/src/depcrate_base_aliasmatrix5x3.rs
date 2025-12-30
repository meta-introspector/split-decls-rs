// Generated macro for Matrix5x3 (type)
macro_rules! Depcrate_base_aliasMatrix5x3 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix5x3"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 5x3 matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix5x3 < T > = Matrix < T , U5 , U3 , ArrayStorage < T , 5 , 3 > > ;
};
}
