// Generated macro for slice_assume_init_mut (function)
macro_rules! Depcrate_next_arrayslice_assume_init_mut {
() => {
// Module: crate::next_array
// Provides: {"slice_assume_init_mut"}
// Dependencies: {}
# [doc = " Assuming all the elements are initialized, get a mutable slice to them."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller guarantees that the elements `T` referenced by `slice` are in a"] # [doc = " valid state."] unsafe fn slice_assume_init_mut < T > (slice : & mut [MaybeUninit < T >]) -> & mut [T] { unsafe { & mut * (slice as * mut [MaybeUninit < T >] as * mut [T]) } }
};
}
