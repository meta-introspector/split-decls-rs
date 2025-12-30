// Generated macro for sha384 (function)
macro_rules! Depcrate_shasha384 {
() => {
// Module: crate::sha
// Provides: {"sha384"}
// Dependencies: {}
# [doc = " Computes the SHA384 hash of some data."] # [corresponds (SHA384)] # [inline] pub fn sha384 (data : & [u8]) -> [u8 ; 48] { unsafe { let mut hash = MaybeUninit :: < [u8 ; 48] > :: uninit () ; ffi :: SHA384 (data . as_ptr () , data . len () , hash . as_mut_ptr () as * mut _) ; hash . assume_init () } }
};
}
