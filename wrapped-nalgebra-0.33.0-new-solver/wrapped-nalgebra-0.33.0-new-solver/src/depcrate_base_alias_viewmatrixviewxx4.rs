// Generated macro for MatrixViewXx4 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx4 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx4"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 4 columns."] # [doc = ""] # [doc = " See [`MatrixViewMutXx4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx4 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U4 , ViewStorage < 'a , T , Dyn , U4 , RStride , CStride > > ;
};
}
