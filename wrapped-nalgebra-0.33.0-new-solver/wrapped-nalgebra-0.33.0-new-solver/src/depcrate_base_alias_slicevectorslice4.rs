// Generated macro for VectorSlice4 (type)
macro_rules! Depcrate_base_alias_sliceVectorSlice4 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSlice4"}
// Dependencies: {}
# [doc = " A 4D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorView4)] pub type VectorSlice4 < 'a , T , RStride = U1 , CStride = U4 > = Matrix < T , U4 , U1 , ViewStorage < 'a , T , U4 , U1 , RStride , CStride > > ;
};
}
