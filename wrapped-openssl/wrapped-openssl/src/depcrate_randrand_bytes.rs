// Generated macro for rand_bytes (function)
macro_rules! Depcrate_randrand_bytes {
() => {
// Module: crate::rand
// Provides: {"rand_bytes"}
// Dependencies: {}
# [doc = " Fill buffer with cryptographically strong pseudo-random bytes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " To generate a buffer with cryptographically strong random bytes:"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::rand::rand_bytes;"] # [doc = ""] # [doc = " let mut buf = [0; 256];"] # [doc = " rand_bytes(&mut buf).unwrap();"] # [doc = " ```"] # [corresponds (RAND_bytes)] pub fn rand_bytes (buf : & mut [u8]) -> Result < () , ErrorStack > { unsafe { ffi :: init () ; assert ! (buf . len () <= c_int :: MAX as usize) ; cvt (ffi :: RAND_bytes (buf . as_mut_ptr () , buf . len () as LenType)) . map (| _ | ()) } }
};
}
