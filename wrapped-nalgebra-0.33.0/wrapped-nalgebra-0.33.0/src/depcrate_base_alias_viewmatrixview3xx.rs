// Generated macro for MatrixView3xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView3xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView3xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 3 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut3xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView3xX < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , Dyn , ViewStorage < 'a , T , U3 , Dyn , RStride , CStride > > ;
};
}
