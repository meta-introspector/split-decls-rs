// Generated macro for impl_125 (impl)
macro_rules! Depcrate_sliceimpl_125 {
() => {
// Module: crate::slice
// Provides: {"impl_125"}
// Dependencies: {}
impl < I : Idx , T , R : IntoSliceIdx < I , [T] > > Index < R > for IndexSlice < I , T > { type Output = < R :: Output as SliceIndex < [T] > > :: Output ; # [inline] fn index (& self , index : R) -> & Self :: Output { & self . raw [index . into_slice_idx ()] } }
};
}
