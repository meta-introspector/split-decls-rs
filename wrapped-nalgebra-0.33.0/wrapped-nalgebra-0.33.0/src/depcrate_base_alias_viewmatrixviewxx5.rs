// Generated macro for MatrixViewXx5 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx5 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx5"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 5 columns."] # [doc = ""] # [doc = " See [`MatrixViewMutXx5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx5 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U5 , ViewStorage < 'a , T , Dyn , U5 , RStride , CStride > > ;
};
}
