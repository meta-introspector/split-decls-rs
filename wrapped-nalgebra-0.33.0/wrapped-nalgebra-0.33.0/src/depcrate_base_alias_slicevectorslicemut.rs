// Generated macro for VectorSliceMut (type)
macro_rules! Depcrate_base_alias_sliceVectorSliceMut {
() => {
// Module: crate::base::alias_slice
// Provides: {"VectorSliceMut"}
// Dependencies: {}
# [doc = " A column vector slice with dimensions known at compile-time."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (VectorViewMut)] pub type VectorSliceMut < 'a , T , D , RStride = U1 , CStride = D > = Matrix < T , D , U1 , ViewStorageMut < 'a , T , D , U1 , RStride , CStride > > ;
};
}
