// Generated macro for Matrix6 (type)
macro_rules! Depcrate_base_aliasMatrix6 {
() => {
// Module: crate::base::alias
// Provides: {"Matrix6"}
// Dependencies: {}
# [doc = " A stack-allocated, column-major, 6x6 square matrix."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type Matrix6 < T > = Matrix < T , U6 , U6 , ArrayStorage < T , 6 , 6 > > ;
};
}
