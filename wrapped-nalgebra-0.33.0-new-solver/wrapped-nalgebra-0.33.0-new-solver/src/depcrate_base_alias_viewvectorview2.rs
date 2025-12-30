// Generated macro for VectorView2 (type)
macro_rules! Depcrate_base_alias_viewVectorView2 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView2"}
// Dependencies: {}
# [doc = " An immutable 2D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut2`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorage < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
