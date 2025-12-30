// Generated macro for derive_next_secret (function)
macro_rules! Depcrate_cryptoderive_next_secret {
() => {
// Module: crate::crypto
// Provides: {"derive_next_secret"}
// Dependencies: {}
fn derive_next_secret (aead : Algorithm , secret : & [u8]) -> Result < Vec < u8 > > { const LABEL : & [u8] = b"quic ku" ; let mut next_secret = vec ! [0u8 ; secret . len ()] ; hkdf_expand_label (aead , secret , LABEL , & mut next_secret) ? ; Ok (next_secret) }
};
}
