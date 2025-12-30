// Generated macro for derive_pkt_key (function)
macro_rules! Depcrate_cryptoderive_pkt_key {
() => {
// Module: crate::crypto
// Provides: {"derive_pkt_key"}
// Dependencies: {}
pub fn derive_pkt_key (aead : Algorithm , prk : & [u8] , out : & mut [u8]) -> Result < () > { const LABEL : & [u8] = b"quic key" ; let key_len : usize = aead . key_len () ; if key_len > out . len () { return Err (Error :: CryptoFail) ; } hkdf_expand_label (aead , prk , LABEL , & mut out [.. key_len]) }
};
}
