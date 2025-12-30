// Generated macro for sha1 (function)
macro_rules! Depcrate_shasha1 {
() => {
// Module: crate::sha
// Provides: {"sha1"}
// Dependencies: {}
# [doc = " Computes the SHA1 hash of some data."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " SHA1 is known to be insecure - it should not be used unless required for"] # [doc = " compatibility with existing systems."] # [corresponds (SHA1)] # [inline] pub fn sha1 (data : & [u8]) -> [u8 ; 20] { unsafe { let mut hash = MaybeUninit :: < [u8 ; 20] > :: uninit () ; ffi :: SHA1 (data . as_ptr () , data . len () , hash . as_mut_ptr () as * mut _) ; hash . assume_init () } }
};
}
