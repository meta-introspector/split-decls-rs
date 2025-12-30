// Generated macro for impl_106 (impl)
macro_rules! Depcrate_idximpl_106 {
() => {
// Module: crate::idx
// Provides: {"impl_106"}
// Dependencies: {}
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeFrom < I > { type Output = ops :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeFrom { start : self . start . index () } } }
};
}
