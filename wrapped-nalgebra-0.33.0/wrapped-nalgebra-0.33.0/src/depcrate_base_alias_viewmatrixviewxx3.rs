// Generated macro for MatrixViewXx3 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx3 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx3"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 3 columns."] # [doc = ""] # [doc = " See [`MatrixViewMutXx3`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx3 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U3 , ViewStorage < 'a , T , Dyn , U3 , RStride , CStride > > ;
};
}
