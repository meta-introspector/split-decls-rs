// Generated macro for SMatrixViewMut (type)
macro_rules! Depcrate_base_alias_viewSMatrixViewMut {
() => {
// Module: crate::base::alias_view
// Provides: {"SMatrixViewMut"}
// Dependencies: {}
# [doc = " A mutable column-major matrix view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`SMatrixView`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type SMatrixViewMut < 'a , T , const R : usize , const C : usize > = Matrix < T , Const < R > , Const < C > , ViewStorageMut < 'a , T , Const < R > , Const < C > , Const < 1 > , Const < R > > > ;
};
}
