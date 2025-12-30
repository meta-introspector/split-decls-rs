// Generated macro for chacha20_poly1305_aead_encrypt (function)
macro_rules! Depcrate_chacha20_poly1305chacha20_poly1305_aead_encrypt {
() => {
// Module: crate::chacha20_poly1305
// Provides: {"chacha20_poly1305_aead_encrypt"}
// Dependencies: {}
# [doc = " Uses the ChaCha20-Poly1305 AEAD algorithm to encrypt `data` in-place"] # [doc = " and sign `data` and `associated_data`, returning the authentication tag."] # [doc = ""] # [doc = " # Errors"] # [doc = " - Returns [StreamWouldExceed256GBError] when `data` is longer than 256GB."] # [allow (clippy :: missing_panics_doc)] pub fn chacha20_poly1305_aead_encrypt (key : & Secret32 , nonce : & Secret12 , associated_data : & [u8] , data : & mut [u8] ,) -> Result < Digest16 , StreamWouldExceed256GBError > { let (mut cipher , mut mac) = new_chacha20_poly1305_aead (key , nonce) ; cipher . apply (data) ? ; mac . update (associated_data , false , false) ; mac . update (data , false , false) ; apply_lengths (& mut mac , associated_data , data) ; Ok (mac . finalize ()) }
};
}
