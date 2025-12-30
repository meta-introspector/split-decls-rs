// Generated macro for MatrixViewXx2 (type)
macro_rules! Depcrate_base_alias_viewMatrixViewXx2 {
() => {
// Module: crate::base::alias_view
// Provides: {"MatrixViewXx2"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with a number of rows chosen at runtime and 2 columns."] # [doc = ""] # [doc = " See [`MatrixViewMutXx2`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type MatrixViewXx2 < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U2 , ViewStorage < 'a , T , Dyn , U2 , RStride , CStride > > ;
};
}
