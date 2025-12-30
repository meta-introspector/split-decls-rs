// Generated macro for MatrixViewXx6 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx6 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx6"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 6 columns."] # [doc = ""] # [doc = " See [`MatrixViewMutXx6`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx6 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U6 , ViewStorage < 'a , T , Dyn , U6 , RStride , CStride > > ;
};
}
