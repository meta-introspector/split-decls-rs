// Generated macro for VectorView4 (type)
macro_rules! Depcrate_base_alias_viewVectorView4 {
() => {
// Module: crate::base::alias_view
// Provides: {"VectorView4"}
// Dependencies: {}
# [doc = " An immutable 4D column vector view."] # [doc = ""] # [doc = " See [`VectorViewMut4`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type VectorView4 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U1 , ViewStorage < 'a , T , U4 , U1 , RStride , CStride > > ;
};
}
