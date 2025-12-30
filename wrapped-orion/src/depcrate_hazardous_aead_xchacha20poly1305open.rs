// Generated macro for open (function)
macro_rules! Depcrate_hazardous_aead_xchacha20poly1305open {
() => {
// Module: crate::hazardous::aead::xchacha20poly1305
// Provides: {"open"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " AEAD XChaCha20Poly1305 decryption as specified in the [draft RFC](https://github.com/bikeshedders/xchacha-rfc)."] pub fn open (secret_key : & SecretKey , nonce : & Nonce , ciphertext_with_tag : & [u8] , ad : Option < & [u8] > , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { let (subkey , ietf_nonce) = subkey_and_nonce (secret_key , nonce) ; chacha20poly1305 :: open (& subkey , & ietf_nonce , ciphertext_with_tag , ad , dst_out) }
};
}
