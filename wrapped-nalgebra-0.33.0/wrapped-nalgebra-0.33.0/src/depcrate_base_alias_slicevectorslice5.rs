// Generated macro for VectorSlice5 (type)
macro_rules! Depcrate_base_alias_sliceVectorSlice5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSlice5"}
// Dependencies: {}
# [doc = " A 5D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorView5)] pub type VectorSlice5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorage < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
