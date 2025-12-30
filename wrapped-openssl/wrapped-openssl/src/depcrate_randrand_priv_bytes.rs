// Generated macro for rand_priv_bytes (function)
macro_rules! Depcrate_randrand_priv_bytes {
() => {
// Module: crate::rand
// Provides: {"rand_priv_bytes"}
// Dependencies: {}
# [doc = " Fill buffer with cryptographically strong pseudo-random bytes. It is"] # [doc = " intended to be used for generating values that should remain private."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " To generate a buffer with cryptographically strong random bytes:"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::rand::rand_priv_bytes;"] # [doc = ""] # [doc = " let mut buf = [0; 256];"] # [doc = " rand_priv_bytes(&mut buf).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Requires OpenSSL 1.1.1 or newer."] # [corresponds (RAND_priv_bytes)] # [cfg (ossl111)] pub fn rand_priv_bytes (buf : & mut [u8]) -> Result < () , ErrorStack > { unsafe { ffi :: init () ; assert ! (buf . len () <= c_int :: MAX as usize) ; cvt (ffi :: RAND_priv_bytes (buf . as_mut_ptr () , buf . len () as LenType)) . map (| _ | ()) } }
};
}
