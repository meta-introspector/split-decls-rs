// Generated macro for DVectorSlice (type)
macro_rules! Depcrate_base_alias_sliceDVectorSlice {
() => {
// Module: crate::base::alias_slice
// Provides: {"DVectorSlice"}
// Dependencies: {}
# [doc = " A column vector slice dynamic numbers of rows and columns."] # [deprecated = slice_deprecation_note ! (DVectorView)] pub type DVectorSlice < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorage < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
