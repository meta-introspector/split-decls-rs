// Generated macro for impl_81 (impl)
macro_rules! Depcrate_buf_uninit_sliceimpl_81 {
() => {
// Module: crate::buf::uninit_slice
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a > From < & 'a mut [u8] > for & 'a mut UninitSlice { fn from (slice : & 'a mut [u8]) -> Self { UninitSlice :: new (slice) } }
};
}
