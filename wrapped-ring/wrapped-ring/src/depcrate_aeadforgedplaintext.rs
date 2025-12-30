// Generated macro for ForgedPlaintext (enum)
macro_rules! Depcrate_aeadForgedPlaintext {
() => {
// Module: crate::aead
// Provides: {"ForgedPlaintext"}
// Dependencies: {}
enum ForgedPlaintext { # [doc = " Zero out the plaintext so that it isn't accidentally leaked or used"] # [doc = " after verification fails. It would be safest if we could check the"] # [doc = " tag before decrypting, but some `open` implementations interleave"] # [doc = " authentication with decryption for performance."] Zero , }
};
}
