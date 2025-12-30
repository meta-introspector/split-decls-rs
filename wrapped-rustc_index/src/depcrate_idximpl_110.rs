// Generated macro for impl_110 (impl)
macro_rules! Depcrate_idximpl_110 {
() => {
// Module: crate::idx
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: Range < I > { type Output = core :: range :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: Range { start : self . start . index () , end : self . end . index () } } }
};
}
