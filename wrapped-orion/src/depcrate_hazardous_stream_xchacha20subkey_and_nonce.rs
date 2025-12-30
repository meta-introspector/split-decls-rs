// Generated macro for subkey_and_nonce (function)
macro_rules! Depcrate_hazardous_stream_xchacha20subkey_and_nonce {
() => {
// Module: crate::hazardous::stream::xchacha20
// Provides: {"subkey_and_nonce"}
// Dependencies: {}
# [doc = " Generate a subkey using HChaCha20 for XChaCha20 and corresponding nonce."] pub (crate) fn subkey_and_nonce (secret_key : & SecretKey , nonce : & Nonce) -> (SecretKey , IETFNonce) { let subkey : SecretKey = SecretKey :: from (chacha20 :: hchacha20 (secret_key , & nonce . as_ref () [0 .. 16]) . unwrap ()) ; let mut prefixed_nonce = [0u8 ; IETF_CHACHA_NONCESIZE] ; prefixed_nonce [4 .. IETF_CHACHA_NONCESIZE] . copy_from_slice (& nonce . as_ref () [16 .. 24]) ; (subkey , IETFNonce :: from (prefixed_nonce)) }
};
}
