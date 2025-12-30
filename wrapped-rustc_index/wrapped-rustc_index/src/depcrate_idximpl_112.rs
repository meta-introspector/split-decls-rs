// Generated macro for impl_112 (impl)
macro_rules! Depcrate_idximpl_112 {
() => {
// Module: crate::idx
// Provides: {"impl_112"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeInclusive < I > { type Output = core :: range :: RangeInclusive < usize > ; # [inline] # [cfg (bootstrap)] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeInclusive { start : self . start . index () , end : self . end . index () } } # [inline] # [cfg (not (bootstrap))] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeInclusive { start : self . start . index () , last : self . last . index () } } }
};
}
