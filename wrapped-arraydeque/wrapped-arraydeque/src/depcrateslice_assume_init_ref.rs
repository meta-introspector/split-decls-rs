// Generated macro for slice_assume_init_ref (function)
macro_rules! Depcrateslice_assume_init_ref {
() => {
// Module: crate
// Provides: {"slice_assume_init_ref"}
// Dependencies: {}
# [doc = " Copy of currently-unstable `MaybeUninit::slice_assume_init_ref`."] unsafe fn slice_assume_init_ref < T > (slice : & [MaybeUninit < T >]) -> & [T] { & * (slice as * const [MaybeUninit < T >] as * const [T]) }
};
}
