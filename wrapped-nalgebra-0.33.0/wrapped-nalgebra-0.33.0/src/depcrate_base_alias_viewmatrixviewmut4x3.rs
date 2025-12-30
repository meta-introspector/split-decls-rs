// Generated macro for MatrixViewMut4x3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut4x3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut4x3"}
// Dependencies: {}
# [doc = " A mutable column-major 4x3 matrix view."] # [doc = ""] # [doc = " See [`MatrixView4x3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut4x3 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U3 , ViewStorageMut < 'a , T , U4 , U3 , RStride , CStride > > ;
};
}
