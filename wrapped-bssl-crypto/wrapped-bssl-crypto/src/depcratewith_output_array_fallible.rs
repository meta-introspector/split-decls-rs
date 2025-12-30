// Generated macro for with_output_array_fallible (function)
macro_rules! Depcratewith_output_array_fallible {
() => {
// Module: crate
// Provides: {"with_output_array_fallible"}
// Dependencies: {}
# [doc = " Wrap a closure that initializes an output buffer and return that buffer as"] # [doc = " an array. The closure returns a [`core::ffi::c_int`] and, if the return value"] # [doc = " is not one, then the initialization is assumed to have failed and [None] is"] # [doc = " returned. Otherwise, this function requires that the closure fully"] # [doc = " initialize the given buffer."] # [doc = ""] # [doc = " Safety: the closure must fully initialize the array if it returns one."] unsafe fn with_output_array_fallible < const N : usize , F > (func : F) -> Option < [u8 ; N] > where F : FnOnce (* mut u8 , usize) -> bool , { let mut out_uninit = core :: mem :: MaybeUninit :: < [u8 ; N] > :: uninit () ; let out_ptr = if N != 0 { out_uninit . as_mut_ptr () as * mut u8 } else { core :: ptr :: null_mut () } ; if func (out_ptr , N) { unsafe { Some (out_uninit . assume_init ()) } } else { None } }
};
}
