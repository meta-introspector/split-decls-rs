// Generated macro for slice_assume_init (function)
macro_rules! Depcrate_header_nameslice_assume_init {
() => {
// Module: crate::header::name
// Provides: {"slice_assume_init"}
// Dependencies: {}
unsafe fn slice_assume_init < T > (slice : & [MaybeUninit < T >]) -> & [T] { & * (slice as * const [MaybeUninit < T >] as * const [T]) }
};
}
