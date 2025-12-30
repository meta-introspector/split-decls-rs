// Generated macro for MatrixView6xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView6xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView6xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 6 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut6xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView6xX < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , Dyn , ViewStorage < 'a , T , U6 , Dyn , RStride , CStride > > ;
};
}
