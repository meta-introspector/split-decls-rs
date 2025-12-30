// Generated macro for impl_113 (impl)
macro_rules! Depcrate_idximpl_113 {
() => {
// Module: crate::idx
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (all (feature = "nightly" , not (bootstrap)))] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeToInclusive < I > { type Output = core :: range :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeToInclusive { last : self . last . index () } } }
};
}
