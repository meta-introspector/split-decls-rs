// Generated macro for chacha20_poly1305_aead_decrypt (function)
macro_rules! Depcrate_chacha20_poly1305chacha20_poly1305_aead_decrypt {
() => {
// Module: crate::chacha20_poly1305
// Provides: {"chacha20_poly1305_aead_decrypt"}
// Dependencies: {}
# [doc = " Uses the ChaCha20-Poly1305 AEAD algorithm to check `authentication_tag` and"] # [doc = " decrypt `data` in-place."] # [doc = ""] # [doc = " # Errors"] # [doc = " - Returns [ChaCha20Poly1305AeadDecryptError::SignatureMismatch] when the signature does not match."] # [doc = " - Returns [ChaCha20Poly1305AeadDecryptError::StreamWouldExceed256GBError]"] # [doc = "   when `data` is longer than 256GB."] # [allow (clippy :: missing_panics_doc)] pub fn chacha20_poly1305_aead_decrypt (key : & Secret32 , nonce : & Secret12 , authentication_tag : & Digest16 , associated_data : & [u8] , data : & mut [u8] ,) -> Result < () , ChaCha20Poly1305AeadDecryptError > { let (mut cipher , mut mac) = new_chacha20_poly1305_aead (key , nonce) ; mac . update (associated_data , false , false) ; mac . update (data , false , false) ; apply_lengths (& mut mac , associated_data , data) ; if ! mac . verify (authentication_tag) { return Err (ChaCha20Poly1305AeadDecryptError :: SignatureMismatch) ; } cipher . apply (data) ? ; Ok (()) }
};
}
