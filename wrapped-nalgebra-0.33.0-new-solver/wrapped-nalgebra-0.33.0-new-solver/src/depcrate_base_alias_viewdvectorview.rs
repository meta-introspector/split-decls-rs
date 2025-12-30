// Generated macro for DVectorView (type)
macro_rules! Depcrate_base_alias_viewDVectorView {
() => {
// Module: crate::base::alias_view
// Provides: {"DVectorView"}
// Dependencies: {}
# [doc = " An immutable column vector view dynamic numbers of rows and columns."] # [doc = ""] # [doc = " See [`DVectorViewMut`] for a mutable version of this type."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type DVectorView < 'a , T , RStride = U1 , CStride = Dyn > = Matrix < T , Dyn , U1 , ViewStorage < 'a , T , Dyn , U1 , RStride , CStride > > ;
};
}
