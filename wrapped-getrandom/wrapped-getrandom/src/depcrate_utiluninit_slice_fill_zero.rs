// Generated macro for uninit_slice_fill_zero (function)
macro_rules! Depcrate_utiluninit_slice_fill_zero {
() => {
// Module: crate::util
// Provides: {"uninit_slice_fill_zero"}
// Dependencies: {}
# [inline] pub fn uninit_slice_fill_zero (slice : & mut [MaybeUninit < u8 >]) -> & mut [u8] { unsafe { ptr :: write_bytes (slice . as_mut_ptr () , 0 , slice . len ()) } ; unsafe { slice_assume_init_mut (slice) } }
};
}
