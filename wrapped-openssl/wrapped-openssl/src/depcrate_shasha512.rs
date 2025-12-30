// Generated macro for sha512 (function)
macro_rules! Depcrate_shasha512 {
() => {
// Module: crate::sha
// Provides: {"sha512"}
// Dependencies: {}
# [doc = " Computes the SHA512 hash of some data."] # [corresponds (SHA512)] # [inline] pub fn sha512 (data : & [u8]) -> [u8 ; 64] { unsafe { let mut hash = MaybeUninit :: < [u8 ; 64] > :: uninit () ; ffi :: SHA512 (data . as_ptr () , data . len () , hash . as_mut_ptr () as * mut _) ; hash . assume_init () } }
};
}
