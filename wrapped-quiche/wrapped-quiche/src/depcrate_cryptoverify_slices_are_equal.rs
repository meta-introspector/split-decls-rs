// Generated macro for verify_slices_are_equal (function)
macro_rules! Depcrate_cryptoverify_slices_are_equal {
() => {
// Module: crate::crypto
// Provides: {"verify_slices_are_equal"}
// Dependencies: {}
pub fn verify_slices_are_equal (a : & [u8] , b : & [u8]) -> Result < () > { if a . len () != b . len () { return Err (Error :: CryptoFail) ; } let rc = unsafe { CRYPTO_memcmp (a . as_ptr () , b . as_ptr () , a . len ()) } ; if rc == 0 { return Ok (()) ; } Err (Error :: CryptoFail) }
};
}
