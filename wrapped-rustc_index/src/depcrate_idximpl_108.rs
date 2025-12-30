// Generated macro for impl_108 (impl)
macro_rules! Depcrate_idximpl_108 {
() => {
// Module: crate::idx
// Provides: {"impl_108"}
// Dependencies: {}
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeInclusive < I > { type Output = ops :: RangeInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeInclusive :: new (self . start () . index () , self . end () . index ()) } }
};
}
