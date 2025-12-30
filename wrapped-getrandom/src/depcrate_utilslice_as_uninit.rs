// Generated macro for slice_as_uninit (function)
macro_rules! Depcrate_utilslice_as_uninit {
() => {
// Module: crate::util
// Provides: {"slice_as_uninit"}
// Dependencies: {}
# [inline (always)] pub fn slice_as_uninit < T > (slice : & [T]) -> & [MaybeUninit < T >] { let ptr = ptr_from_ref :: < [T] > (slice) as * const [MaybeUninit < T >] ; unsafe { & * ptr } }
};
}
