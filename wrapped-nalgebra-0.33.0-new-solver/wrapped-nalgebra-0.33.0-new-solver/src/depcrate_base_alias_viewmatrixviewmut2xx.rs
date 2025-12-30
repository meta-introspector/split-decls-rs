// Generated macro for MatrixViewMut2xX (type)
macro_rules! Depcrate_base_alias_viewMatrixViewMut2xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewMut2xX"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with 2 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixView2xX`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewMut2xX < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , Dyn , ViewStorageMut < 'a , T , U2 , Dyn , RStride , CStride > > ;
};
}
