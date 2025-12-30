// Generated macro for Matrix2x3 (type)
macro_rules! Depcrate_base_aliasMatrix2x3 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix2x3"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 2x3 matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix2x3 < T > = Matrix < T , U2 , U3 , ArrayStorage < T , 2 , 3 > > ;
};
}
