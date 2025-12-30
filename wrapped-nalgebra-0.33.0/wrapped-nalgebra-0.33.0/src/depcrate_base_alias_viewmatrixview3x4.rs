// Generated macro for MatrixView3x4 (type)
macro_rules! Depcrate_base_alias_viewMatrixView3x4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView3x4"}
// Dependencies: {}
# [doc = " An immutable column-major 3x4 matrix view."] # [doc = ""] # [doc = " See [`MatrixViewMut3x4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView3x4 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U4 , ViewStorage < 'a , T , U3 , U4 , RStride , CStride > > ;
};
}
