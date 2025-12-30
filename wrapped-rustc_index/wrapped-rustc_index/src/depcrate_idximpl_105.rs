// Generated macro for impl_105 (impl)
macro_rules! Depcrate_idximpl_105 {
() => {
// Module: crate::idx
// Provides: {"impl_105"}
// Dependencies: {}
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: Range < I > { type Output = ops :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: Range { start : self . start . index () , end : self . end . index () } } }
};
}
