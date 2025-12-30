// Generated macro for MatrixView4xX (type)
macro_rules! Depcrate_base_alias_viewMatrixView4xX {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixView4xX"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with 4 rows and a number of columns chosen at runtime."] # [doc = ""] # [doc = " See [`MatrixViewMut4xX`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixView4xX < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , Dyn , ViewStorage < 'a , T , U4 , Dyn , RStride , CStride > > ;
};
}
