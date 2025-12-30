// Generated macro for SVectorView (type)
macro_rules! Depcrate_base_alias_viewSVectorView {
() => {
// Module: crate::base::alias_view
// Provides: {"SVectorView"}
// Dependencies: {}
# [doc = " An immutable column vector view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`SVectorViewMut`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type SVectorView < 'a , T , const D : usize > = Matrix < T , Const < D > , Const < 1 > , ViewStorage < 'a , T , Const < D > , Const < 1 > , Const < 1 > , Const < D > > > ;
};
}
