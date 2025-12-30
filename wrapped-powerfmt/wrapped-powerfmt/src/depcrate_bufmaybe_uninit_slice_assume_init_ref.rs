// Generated macro for maybe_uninit_slice_assume_init_ref (function)
macro_rules! Depcrate_bufmaybe_uninit_slice_assume_init_ref {
() => {
// Module: crate::buf
// Provides: {"maybe_uninit_slice_assume_init_ref"}
// Dependencies: {}
# [doc = " Equivalent of [`MaybeUninit::slice_assume_init_ref`] that compiles on stable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See [`MaybeUninit::slice_assume_init_ref`](https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#method.slice_assume_init_ref)."] # [inline (always)] const unsafe fn maybe_uninit_slice_assume_init_ref < T > (slice : & [MaybeUninit < T >]) -> & [T] { # [allow (trivial_casts)] unsafe { & * (slice as * const [MaybeUninit < T >] as * const [T]) } }
};
}
