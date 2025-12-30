// Generated macro for SMatrixView (type)
macro_rules! Depcrate_base_alias_viewSMatrixView {
() => {
// Module: crate::base::alias_view
// Provides: {"SMatrixView"}
// Dependencies: {}
# [doc = " An immutable column-major matrix view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`SMatrixViewMut`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type SMatrixView < 'a , T , const R : usize , const C : usize > = Matrix < T , Const < R > , Const < C > , ViewStorage < 'a , T , Const < R > , Const < C > , Const < 1 > , Const < R > > > ;
};
}
