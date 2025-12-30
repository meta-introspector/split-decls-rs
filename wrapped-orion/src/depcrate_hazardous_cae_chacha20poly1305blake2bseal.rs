// Generated macro for seal (function)
macro_rules! Depcrate_hazardous_cae_chacha20poly1305blake2bseal {
() => {
// Module: crate::hazardous::cae::chacha20poly1305blake2b
// Provides: {"seal"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " CTX ChaCha20Poly1305 with BLAKE2b-256."] pub fn seal (secret_key : & SecretKey , nonce : & Nonce , plaintext : & [u8] , ad : Option < & [u8] > , dst_out : & mut [u8] ,) -> Result < () , UnknownCryptoError > { if u64 :: try_from (plaintext . len ()) . map_err (| _ | UnknownCryptoError) ? > P_MAX { return Err (UnknownCryptoError) ; } let ad = ad . unwrap_or (& [0u8 ; 0]) ; # [allow (clippy :: absurd_extreme_comparisons)] if u64 :: try_from (ad . len ()) . map_err (| _ | UnknownCryptoError) ? > A_MAX { return Err (UnknownCryptoError) ; } match plaintext . len () . checked_add (TAG_SIZE) { Some (out_min_len) => { if dst_out . len () < out_min_len { return Err (UnknownCryptoError) ; } } None => return Err (UnknownCryptoError) , } ; aead :: chacha20poly1305 :: seal (secret_key , nonce , plaintext , Some (ad) , & mut dst_out [.. plaintext . len () + POLY1305_OUTSIZE] ,) ? ; let mut blake2b = Blake2b :: new (32) ? ; blake2b . update (secret_key . unprotected_as_bytes ()) ? ; blake2b . update (nonce . as_ref ()) ? ; blake2b . update (ad) ? ; blake2b . update (& dst_out [plaintext . len () .. plaintext . len () + POLY1305_OUTSIZE]) ? ; let tag = blake2b . finalize () ? ; dst_out [plaintext . len () .. plaintext . len () + TAG_SIZE] . copy_from_slice (tag . as_ref ()) ; Ok (()) }
};
}
