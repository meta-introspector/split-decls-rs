// Generated macro for VectorViewMut3 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut3 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut3"}
// Dependencies: {}
# [doc = " A mutable 3D column vector view."] # [doc = ""] # [doc = " See [`VectorView3`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U1 , ViewStorageMut < 'a , T , U3 , U1 , RStride , CStride > > ;
};
}
