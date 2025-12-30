// Generated macro for impl_107 (impl)
macro_rules! Depcrate_idximpl_107 {
() => {
// Module: crate::idx
// Provides: {"impl_107"}
// Dependencies: {}
impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeTo < I > { type Output = ops :: RangeTo < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { .. self . end . index () } }
};
}
