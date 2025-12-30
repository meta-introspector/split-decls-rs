// Generated macro for Matrix2 (type)
macro_rules! Depcrate_base_aliasMatrix2 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix2"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 2x2 square matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix2 < T > = Matrix < T , U2 , U2 , ArrayStorage < T , 2 , 2 > > ;
};
}
