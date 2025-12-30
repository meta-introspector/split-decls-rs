// Generated macro for VectorView (type)
macro_rules! Depcrate_base_alias_viewVectorView {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView"}
// Dependencies: {}
# [doc = " An immutable column vector view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`VectorViewMut`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView < 'a , T , D , RStride = U1 , CStride = D > = Matrix < T , D , U1 , ViewStorage < 'a , T , D , U1 , RStride , CStride > > ;
};
}
