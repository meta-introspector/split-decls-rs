// Generated macro for VectorSlice6 (type)
macro_rules! Depcrate_base_alias_sliceVectorSlice6 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSlice6"}
// Dependencies: {}
# [doc = " A 6D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorView6)] pub type VectorSlice6 < 'a , T , RStride = U1 , CStride = U6 > = Matrix < T , U6 , U1 , ViewStorage < 'a , T , U6 , U1 , RStride , CStride > > ;
};
}
