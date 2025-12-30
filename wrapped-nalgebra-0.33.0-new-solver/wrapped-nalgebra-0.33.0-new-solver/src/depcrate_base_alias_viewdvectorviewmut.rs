// Generated macro for DVectorViewMut (type)
macro_rules! Depcrate_base_alias_viewDVectorViewMut {
() => {
// Module: crate::base::alias_view
// Provides: {"DVectorViewMut"}
// Dependencies: {}
# [doc = " A mutable column vector view dynamic numbers of rows and columns."] # [doc = ""] # [doc = " See [`DVectorView`] for an immutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type DVectorViewMut < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorageMut < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
