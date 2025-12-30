// Generated macro for TestVector (struct)
macro_rules! Depcrate_devTestVector {
() => {
// Module: crate::dev
// Provides: {"TestVector"}
// Dependencies: {}
# [doc = " AEAD test vector"] # [derive (Debug , Clone , Copy)] pub struct TestVector { # [doc = " Initialization key"] pub key : & 'static [u8] , # [doc = " Nonce"] pub nonce : & 'static [u8] , # [doc = " Additional associated data"] pub aad : & 'static [u8] , # [doc = " Plaintext"] pub plaintext : & 'static [u8] , # [doc = " Ciphertext"] pub ciphertext : & 'static [u8] , }
};
}
