// Generated macro for VectorViewMut6 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut6 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut6"}
// Dependencies: {}
# [doc = " A mutable 6D column vector view."] # [doc = ""] # [doc = " See [`VectorView6`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorageMut < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
