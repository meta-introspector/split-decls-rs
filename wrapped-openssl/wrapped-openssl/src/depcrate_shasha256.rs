// Generated macro for sha256 (function)
macro_rules! Depcrate_shasha256 {
() => {
// Module: crate::sha
// Provides: {"sha256"}
// Dependencies: {}
# [doc = " Computes the SHA256 hash of some data."] # [corresponds (SHA256)] # [inline] pub fn sha256 (data : & [u8]) -> [u8 ; 32] { unsafe { let mut hash = MaybeUninit :: < [u8 ; 32] > :: uninit () ; ffi :: SHA256 (data . as_ptr () , data . len () , hash . as_mut_ptr () as * mut _) ; hash . assume_init () } }
};
}
