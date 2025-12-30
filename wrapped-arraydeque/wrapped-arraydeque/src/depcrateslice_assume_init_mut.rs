// Generated macro for slice_assume_init_mut (function)
macro_rules! Depcrateslice_assume_init_mut {
() => {
// Module: crate
// Provides: {"slice_assume_init_mut"}
// Dependencies: {}
# [doc = " Copy of currently-unstable `MaybeUninit::slice_assume_init_mut`."] unsafe fn slice_assume_init_mut < T > (slice : & mut [MaybeUninit < T >]) -> & mut [T] { & mut * (slice as * mut [MaybeUninit < T >] as * mut [T]) }
};
}
