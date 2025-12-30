// Generated macro for MatrixViewXx1 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx1 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx1"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 1 column."] # [doc = ""] # [doc = " See [`MatrixViewMutXx1`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx1 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorage < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
