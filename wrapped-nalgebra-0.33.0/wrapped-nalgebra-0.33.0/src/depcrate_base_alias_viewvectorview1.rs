// Generated macro for VectorView1 (type)
macro_rules! Depcrate_base_alias_viewVectorView1 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView1"}
// Dependencies: {}
# [doc = " An immutable 1D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut1`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView1 < 'a , T , RStride = U1 , CStride = U1 > = Matrix < T , U1 , U1 , ViewStorage < 'a , T , U1 , U1 , RStride , CStride > > ;
};
}
