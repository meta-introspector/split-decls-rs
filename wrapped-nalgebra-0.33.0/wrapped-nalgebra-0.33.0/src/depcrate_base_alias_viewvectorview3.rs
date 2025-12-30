// Generated macro for VectorView3 (type)
macro_rules! Depcrate_base_alias_viewVectorView3 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView3"}
// Dependencies: {}
# [doc = " An immutable 3D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut3`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U1 , ViewStorage < 'a , T , U3 , U1 , RStride , CStride > > ;
};
}
