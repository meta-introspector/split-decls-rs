// Generated macro for VectorView5 (type)
macro_rules! Depcrate_base_alias_viewVectorView5 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView5"}
// Dependencies: {}
# [doc = " An immutable 5D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut5`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorage < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
