// Generated macro for slice_assume_init_mut (function)
macro_rules! Depcrate_utilslice_assume_init_mut {
() => {
// Module: crate::util
// Provides: {"slice_assume_init_mut"}
// Dependencies: {}
# [doc = " Polyfill for `maybe_uninit_slice` feature's"] # [doc = " `MaybeUninit::slice_assume_init_mut`. Every element of `slice` must have"] # [doc = " been initialized."] # [inline (always)] # [allow (unused_unsafe)] pub unsafe fn slice_assume_init_mut < T > (slice : & mut [MaybeUninit < T >]) -> & mut [T] { let ptr = ptr_from_mut :: < [MaybeUninit < T >] > (slice) as * mut [T] ; unsafe { & mut * ptr } }
};
}
