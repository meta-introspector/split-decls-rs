// Generated macro for VectorSliceMut5 (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut5 {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut5"}
// Dependencies: {}
# [doc = " A 5D column vector slice."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut5)] pub type VectorSliceMut5 < 'a , T , RStride = U1 , CStride = U5 > = Matrix < T , U5 , U1 , ViewStorageMut < 'a , T , U5 , U1 , RStride , CStride > > ;
};
}
