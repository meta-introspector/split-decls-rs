// Generated macro for VectorViewMut (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut"}
// Dependencies: {}
# [doc = " A mutable column vector view with dimensions known at compile-time."] # [doc = ""] # [doc = " See [`VectorView`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut < 'a , T , D , RStride = U1 , CStride = D > = Matrix < T , D , U1 , ViewStorageMut < 'a , T , D , U1 , RStride , CStride > > ;
};
}
