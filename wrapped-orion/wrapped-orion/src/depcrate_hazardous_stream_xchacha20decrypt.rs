// Generated macro for decrypt (function)
macro_rules! Depcrate_hazardous_stream_xchacha20decrypt {
() => {
// Module: crate::hazardous::stream::xchacha20
// Provides: {"decrypt"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " XChaCha20 decryption as specified in the [draft RFC](https://tools.ietf.org/html/draft-irtf-cfrg-xchacha-03)."] pub fn decrypt (secret_key : & SecretKey , nonce : & Nonce , initial_counter : u32 , ciphertext : & [u8] , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { encrypt (secret_key , nonce , initial_counter , ciphertext , dst_out) }
};
}
