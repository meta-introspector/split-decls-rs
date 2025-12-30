// Generated macro for deinit_slice_mut (function)
macro_rules! Depcratedeinit_slice_mut {
() => {
// Module: crate
// Provides: {"deinit_slice_mut"}
// Dependencies: {}
unsafe fn deinit_slice_mut < 'a , 'b , T > (s : & 'a mut & 'b mut [T]) -> & 'a mut & 'b mut [MaybeUninit < T >] { let s : * mut & mut [T] = s ; let s = s as * mut & mut [MaybeUninit < T >] ; & mut * s }
};
}
