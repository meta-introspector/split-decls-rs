// Generated macro for MatrixView5xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView5xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView5xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 5 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut5xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView5xX < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , Dyn , ViewStorage < 'a , T , U5 , Dyn , RStride , CStride > > ;
};
}
