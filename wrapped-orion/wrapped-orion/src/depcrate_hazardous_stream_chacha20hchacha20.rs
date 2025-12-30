// Generated macro for hchacha20 (function)
macro_rules! Depcrate_hazardous_stream_chacha20hchacha20 {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"hchacha20"}
// Dependencies: {}
# [doc = " HChaCha20 as specified in the [draft-RFC](https://github.com/bikeshedders/xchacha-rfc/blob/master)."] pub (super) fn hchacha20 (secret_key : & SecretKey , nonce : & [u8] ,) -> Result < [u8 ; HCHACHA_OUTSIZE] , UnknownCryptoError > { let mut chacha_state = ChaCha20 :: new (secret_key . unprotected_as_bytes () , nonce , false) ? ; let mut keystream_block = [0u8 ; HCHACHA_OUTSIZE] ; chacha_state . keystream_block (0 , & mut keystream_block) ; Ok (keystream_block) }
};
}
