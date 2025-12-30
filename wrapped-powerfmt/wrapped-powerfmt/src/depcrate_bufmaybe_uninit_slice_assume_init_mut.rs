// Generated macro for maybe_uninit_slice_assume_init_mut (function)
macro_rules! Depcrate_bufmaybe_uninit_slice_assume_init_mut {
() => {
// Module: crate::buf
// Provides: {"maybe_uninit_slice_assume_init_mut"}
// Dependencies: {}
# [doc = " Equivalent of [`MaybeUninit::slice_assume_init_mut`] that compiles on stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`MaybeUninit::slice_assume_init_mut`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.slice_assume_init_mut)."] # [inline (always)] unsafe fn maybe_uninit_slice_assume_init_mut < T , U > (slice : & mut [MaybeUninit < T >]) -> & mut [U] { # [allow (trivial_casts)] unsafe { & mut * (slice as * mut [MaybeUninit < T >] as * mut [U]) } }
};
}
