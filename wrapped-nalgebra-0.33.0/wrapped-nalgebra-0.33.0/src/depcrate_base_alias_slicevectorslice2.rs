// Generated macro for VectorSlice2 (type)
macro_rules! Depcrate_base_alias_sliceVectorSlice2 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSlice2"}
// Dependencies: {}
# [doc = " A 2D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorView2)] pub type VectorSlice2 < 'a , T , RStride = U1 , CStride = U2 > = Matrix < T , U2 , U1 , ViewStorage < 'a , T , U2 , U1 , RStride , CStride > > ;
};
}
