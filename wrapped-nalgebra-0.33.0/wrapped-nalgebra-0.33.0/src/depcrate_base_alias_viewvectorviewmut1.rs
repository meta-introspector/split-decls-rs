// Generated macro for VectorViewMut1 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut1 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut1"}
// Dependencies: {}
# [doc = " A mutable 1D column vector view."] # [doc = ""] # [doc = " See [`VectorView1`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut1 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U1 , ViewStorageMut < 'a , T , U1 , U1 , RStride , CStride > > ;
};
}
