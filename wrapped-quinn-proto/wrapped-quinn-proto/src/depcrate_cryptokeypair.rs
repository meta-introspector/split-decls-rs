// Generated macro for KeyPair (struct)
macro_rules! Depcrate_cryptoKeyPair {
() => {
// Module: crate::crypto
// Provides: {"KeyPair"}
// Dependencies: {}
# [doc = " A pair of keys for bidirectional communication"] pub struct KeyPair < T > { # [doc = " Key for encrypting data"] pub local : T , # [doc = " Key for decrypting data"] pub remote : T , }
};
}
