// Generated macro for impl_109 (impl)
macro_rules! Depcrate_idximpl_109 {
() => {
// Module: crate::idx
// Provides: {"impl_109"}
// Dependencies: {}
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeToInclusive < I > { type Output = ops :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ..= self . end . index () } }
};
}
