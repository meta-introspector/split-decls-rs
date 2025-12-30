// Generated macro for impl_111 (impl)
macro_rules! Depcrate_idximpl_111 {
() => {
// Module: crate::idx
// Provides: {"impl_111"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeFrom < I > { type Output = core :: range :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeFrom { start : self . start . index () } } }
};
}
