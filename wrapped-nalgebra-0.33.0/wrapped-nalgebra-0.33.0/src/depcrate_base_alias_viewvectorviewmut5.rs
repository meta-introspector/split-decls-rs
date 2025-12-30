// Generated macro for VectorViewMut5 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut5 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut5"}
// Dependencies: {}
# [doc = " A mutable 5D column vector view."] # [doc = ""] # [doc = " See [`VectorView5`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorageMut < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
