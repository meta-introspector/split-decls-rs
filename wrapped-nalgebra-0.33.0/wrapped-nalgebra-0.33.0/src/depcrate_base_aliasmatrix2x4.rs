// Generated macro for Matrix2x4 (type)
macro_rules! Depcrate_base_aliasMatrix2x4 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix2x4"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 2x4 matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix2x4 < T > = Matrix < T , U2 , U4 , ArrayStorage < T , 2 , 4 > > ;
};
}
