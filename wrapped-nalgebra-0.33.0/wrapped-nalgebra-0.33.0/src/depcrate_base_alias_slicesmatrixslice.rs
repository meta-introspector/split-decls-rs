// Generated macro for SMatrixSlice (type)
macro_rules! Depcrate_base_alias_sliceSMatrixSlice {
() => {
// Module: crate::base::alias_slice
// Provides: {"SMatrixSlice"}
// Dependencies: {}
# [doc = " A column-major matrix slice with dimensions known at compile-time."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated = slice_deprecation_note ! (SMatrixView)] pub type SMatrixSlice < 'a , T , const R : usize , const C : usize > = Matrix < T , Const < R > , Const < C > , ViewStorage < 'a , T , Const < R > , Const < C > , Const < 1 > , Const < R > > > ;
};
}
