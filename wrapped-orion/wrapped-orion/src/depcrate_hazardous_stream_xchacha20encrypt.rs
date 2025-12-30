// Generated macro for encrypt (function)
macro_rules! Depcrate_hazardous_stream_xchacha20encrypt {
() => {
// Module: crate::hazardous::stream::xchacha20
// Provides: {"encrypt"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " XChaCha20 encryption as specified in the [draft RFC](https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-03)."] pub fn encrypt (secret_key : & SecretKey , nonce : & Nonce , initial_counter : u32 , plaintext : & [u8] , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { let (subkey , ietf_nonce) = subkey_and_nonce (secret_key , nonce) ; chacha20 :: encrypt (& subkey , & ietf_nonce , initial_counter , plaintext , dst_out) }
};
}
