// Generated macro for Matrix5 (type)
macro_rules! Depcrate_base_aliasMatrix5 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix5"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 5x5 square matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix5 < T > = Matrix < T , U5 , U5 , ArrayStorage < T , 5 , 5 > > ;
};
}
