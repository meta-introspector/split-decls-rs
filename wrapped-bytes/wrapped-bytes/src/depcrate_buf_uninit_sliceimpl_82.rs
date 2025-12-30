// Generated macro for impl_82 (impl)
macro_rules! Depcrate_buf_uninit_sliceimpl_82 {
() => {
// Module: crate::buf::uninit_slice
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a > From < & 'a mut [MaybeUninit < u8 >] > for & 'a mut UninitSlice { fn from (slice : & 'a mut [MaybeUninit < u8 >]) -> Self { UninitSlice :: uninit (slice) } }
};
}
