// Generated macro for DVectorSliceMut (type)
macro_rules! Depcrate_base_alias_sliceDVectorSliceMut {
() => {
// Module: crate::base::alias_slice
// Provides: {"DVectorSliceMut"}
// Dependencies: {}
# [doc = " A column vector slice dynamic numbers of rows and columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (DVectorViewMut)] pub type DVectorSliceMut < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorageMut < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
