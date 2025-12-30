// Generated macro for derive_hdr_key (function)
macro_rules! Depcrate_cryptoderive_hdr_key {
() => {
// Module: crate::crypto
// Provides: {"derive_hdr_key"}
// Dependencies: {}
pub fn derive_hdr_key (aead : Algorithm , secret : & [u8] , out : & mut [u8] ,) -> Result < () > { const LABEL : & [u8] = b"quic hp" ; let key_len = aead . key_len () ; if key_len > out . len () { return Err (Error :: CryptoFail) ; } hkdf_expand_label (aead , secret , LABEL , & mut out [.. key_len]) }
};
}
