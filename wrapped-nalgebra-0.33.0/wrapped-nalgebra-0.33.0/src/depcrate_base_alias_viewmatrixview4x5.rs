// Generated macro for MatrixView4x5 (type)
macro_rules! Depcrate_base_alias_viewMatrixView4x5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView4x5"}
// Dependencies: {}
# [doc = " An immutable column-major 4x5 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut4x5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView4x5 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U5 , ViewStorage < 'a , T , U4 , U5 , RStride , CStride > > ;
};
}
