// Generated macro for sha224 (function)
macro_rules! Depcrate_shasha224 {
() => {
// Module: crate::sha
// Provides: {"sha224"}
// Dependencies: {}
# [doc = " Computes the SHA224 hash of some data."] # [corresponds (SHA224)] # [inline] pub fn sha224 (data : & [u8]) -> [u8 ; 28] { unsafe { let mut hash = MaybeUninit :: < [u8 ; 28] > :: uninit () ; ffi :: SHA224 (data . as_ptr () , data . len () , hash . as_mut_ptr () as * mut _) ; hash . assume_init () } }
};
}
