// Generated macro for VectorViewMut4 (type)
macro_rules! Depcrate_base_alias_viewVectorViewMut4 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorViewMut4"}
// Dependencies: {}
# [doc = " A mutable 4D column vector view."] # [doc = ""] # [doc = " See [`VectorView4`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorViewMut4 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U1 , ViewStorageMut < 'a , T , U4 , U1 , RStride , CStride > > ;
};
}
