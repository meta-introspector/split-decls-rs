// Generated macro for derive_pkt_iv (function)
macro_rules! Depcrate_cryptoderive_pkt_iv {
() => {
// Module: crate::crypto
// Provides: {"derive_pkt_iv"}
// Dependencies: {}
pub fn derive_pkt_iv (aead : Algorithm , prk : & [u8] , out : & mut [u8]) -> Result < () > { const LABEL : & [u8] = b"quic iv" ; let nonce_len = aead . nonce_len () ; if nonce_len > out . len () { return Err (Error :: CryptoFail) ; } hkdf_expand_label (aead , prk , LABEL , & mut out [.. nonce_len]) }
};
}
