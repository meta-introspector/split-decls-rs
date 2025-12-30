// Generated macro for VectorView6 (type)
macro_rules! Depcrate_base_alias_viewVectorView6 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView6"}
// Dependencies: {}
# [doc = " An immutable 6D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut6`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorage < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
