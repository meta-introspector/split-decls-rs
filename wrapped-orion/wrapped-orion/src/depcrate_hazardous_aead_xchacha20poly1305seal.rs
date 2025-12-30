// Generated macro for seal (function)
macro_rules! Depcrate_hazardous_aead_xchacha20poly1305seal {
() => {
// Module: crate::hazardous::aead::xchacha20poly1305
// Provides: {"seal"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " AEAD XChaCha20Poly1305 encryption as specified in the [draft RFC](https://github.com/bikeshedders/xchacha-rfc)."] pub fn seal (secret_key : & SecretKey , nonce : & Nonce , plaintext : & [u8] , ad : Option < & [u8] > , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { let (subkey , ietf_nonce) = subkey_and_nonce (secret_key , nonce) ; chacha20poly1305 :: seal (& subkey , & ietf_nonce , plaintext , ad , dst_out) }
};
}
