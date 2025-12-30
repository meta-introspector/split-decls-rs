// Generated macro for VectorViewMut2 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut2 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut2"}
// Dependencies: {}
# [doc = " A mutable 2D column vector view."] # [doc = ""] # [doc = " See [`VectorView2`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorageMut < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
