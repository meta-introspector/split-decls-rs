// Generated macro for assume_init_slice (function)
macro_rules! Depcrateassume_init_slice {
() => {
// Module: crate
// Provides: {"assume_init_slice"}
// Dependencies: {}
unsafe fn assume_init_slice < T > (s : & mut [MaybeUninit < T >]) -> & mut [T] { let s : * mut [MaybeUninit < T >] = s ; let s = s as * mut [T] ; & mut * s }
};
}
