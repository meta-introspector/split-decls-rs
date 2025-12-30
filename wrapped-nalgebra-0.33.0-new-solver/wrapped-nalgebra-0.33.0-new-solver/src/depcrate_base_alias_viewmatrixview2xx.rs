// Generated macro for MatrixView2xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView2xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView2xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 2 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut2xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView2xX < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , Dyn , ViewStorage < 'a , T , U2 , Dyn , RStride , CStride > > ;
};
}
