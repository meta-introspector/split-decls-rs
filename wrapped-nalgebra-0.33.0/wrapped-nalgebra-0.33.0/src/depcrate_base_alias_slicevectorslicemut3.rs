// Generated macro for VectorSliceMut3 (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut3 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut3"}
// Dependencies: {}
# [doc = " A 3D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut3)] pub type VectorSliceMut3 < 'a , T , RStride = U1 , CStride = U3 > = Matrix < T , U3 , U1 , ViewStorageMut < 'a , T , U3 , U1 , RStride , CStride > > ;
};
}
