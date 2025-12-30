// Generated macro for VectorSlice3 (type)
macro_rules! Depcrate_base_alias_sliceVectorSlice3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSlice3"}
// Dependencies: {}
# [doc = " A 3D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorView3)] pub type VectorSlice3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U1 , ViewStorage < 'a , T , U3 , U1 , RStride , CStride > > ;
};
}
