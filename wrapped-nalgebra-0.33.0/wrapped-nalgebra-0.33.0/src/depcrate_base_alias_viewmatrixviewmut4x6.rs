// Generated macro for MatrixViewMut4x6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut4x6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut4x6"}
// Dependencies: {}
# [doc = " A mutable column-major 4x6 matrix view."] # [doc = ""] # [doc = " See [`MatrixView4x6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut4x6 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U6 , ViewStorageMut < 'a , T , U4 , U6 , RStride , CStride > > ;
};
}
