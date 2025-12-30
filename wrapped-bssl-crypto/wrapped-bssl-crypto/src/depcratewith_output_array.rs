// Generated macro for with_output_array (function)
macro_rules! Depcratewith_output_array {
() => {
// Module: crate
// Provides: {"with_output_array"}
// Dependencies: {}
# [doc = " Wrap a closure that initializes an output buffer and return that buffer as"] # [doc = " an array. Requires that the closure fully initialize the given buffer."] # [doc = ""] # [doc = " Safety: the closure must fully initialize the array."] unsafe fn with_output_array < const N : usize , F > (func : F) -> [u8 ; N] where F : FnOnce (* mut u8 , usize) , { let mut out_uninit = core :: mem :: MaybeUninit :: < [u8 ; N] > :: uninit () ; let out_ptr = if N != 0 { out_uninit . as_mut_ptr () as * mut u8 } else { core :: ptr :: null_mut () } ; func (out_ptr , N) ; unsafe { out_uninit . assume_init () } }
};
}
