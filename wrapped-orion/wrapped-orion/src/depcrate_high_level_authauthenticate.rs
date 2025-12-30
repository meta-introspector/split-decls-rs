// Generated macro for authenticate (function)
macro_rules! Depcrate_high_level_authauthenticate {
() => {
// Module: crate::high_level::auth
// Provides: {"authenticate"}
// Dependencies: {}
# [must_use = "SECURITY WARNING: Ignoring a Result can have real security implications."] # [doc = " Authenticate a message using BLAKE2b-256 in keyed mode."] pub fn authenticate (secret_key : & SecretKey , data : & [u8]) -> Result < Tag , UnknownCryptoError > { if secret_key . len () < BLAKE2B_MIN_KEY_SIZE { return Err (UnknownCryptoError) ; } let blake2b_secret_key = blake2b :: SecretKey :: from_slice (secret_key . unprotected_as_bytes ()) ? ; let mut state = Blake2b :: new (& blake2b_secret_key , BLAKE2B_TAG_SIZE) ? ; state . update (data) ? ; state . finalize () }
};
}
