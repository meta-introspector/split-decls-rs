// Generated macro for SVectorViewMut (type)
macro_rules! Depcrate_base_alias_viewSVectorViewMut {
() => {
// Module: crate::base::alias_view
// Provides: {"SVectorViewMut"}
// Dependencies: {}
# [doc = " A mutable column vector view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`SVectorView`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type SVectorViewMut < 'a , T , const D : usize > = Matrix < T , Const < D > , Const < 1 > , ViewStorageMut < 'a , T , Const < D > , Const < 1 > , Const < 1 > , Const < D > > > ;
};
}
