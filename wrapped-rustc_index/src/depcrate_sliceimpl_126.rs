// Generated macro for impl_126 (impl)
macro_rules! Depcrate_sliceimpl_126 {
() => {
// Module: crate::slice
// Provides: {"impl_126"}
// Dependencies: {}
impl < I : Idx , T , R : IntoSliceIdx < I , [T] > > IndexMut < R > for IndexSlice < I , T > { # [inline] fn index_mut (& mut self , index : R) -> & mut Self :: Output { & mut self . raw [index . into_slice_idx ()] } }
};
}
