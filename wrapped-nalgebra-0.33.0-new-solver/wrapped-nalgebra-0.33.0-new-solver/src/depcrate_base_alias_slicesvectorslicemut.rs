// Generated macro for SVectorSliceMut (type)
macro_rules! Depcrate_base_alias_sliceSVectorSliceMut {
() => {
// Module: crate::base::alias_slice
// Provides: {"SVectorSliceMut"}
// Dependencies: {}
# [doc = " A column vector slice with dimensions known at compile-time."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (SVectorViewMut)] pub type SVectorSliceMut < 'a , T , const D : usize > = Matrix < T , Const < D > , Const < 1 > , ViewStorageMut < 'a , T , Const < D > , Const < 1 > , Const < 1 > , Const < D > > > ;
};
}
