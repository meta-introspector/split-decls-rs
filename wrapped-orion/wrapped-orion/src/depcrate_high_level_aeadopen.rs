// Generated macro for open (function)
macro_rules! Depcrate_high_level_aeadopen {
() => {
// Module: crate::high_level::aead
// Provides: {"open"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Authenticated decryption using XChaCha20Poly1305."] pub fn open (secret_key : & SecretKey , ciphertext_with_tag_and_nonce : & [u8] ,) -> Result < Vec < u8 > , UnknownCryptoError > { if ciphertext_with_tag_and_nonce . len () <= (XCHACHA_NONCESIZE + POLY1305_OUTSIZE) { return Err (UnknownCryptoError) ; } let mut dst_out = vec ! [0u8 ; ciphertext_with_tag_and_nonce . len () - (XCHACHA_NONCESIZE + POLY1305_OUTSIZE)] ; aead :: xchacha20poly1305 :: open (& chacha20 :: SecretKey :: from_slice (secret_key . unprotected_as_bytes ()) ? , & Nonce :: from_slice (& ciphertext_with_tag_and_nonce [.. XCHACHA_NONCESIZE]) ? , & ciphertext_with_tag_and_nonce [XCHACHA_NONCESIZE ..] , None , & mut dst_out ,) ? ; Ok (dst_out) }
};
}
