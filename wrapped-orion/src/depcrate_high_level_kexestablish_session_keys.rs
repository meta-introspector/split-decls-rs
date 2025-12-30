// Generated macro for establish_session_keys (function)
macro_rules! Depcrate_high_level_kexestablish_session_keys {
() => {
// Module: crate::high_level::kex
// Provides: {"establish_session_keys"}
// Dependencies: {}
# [doc = " Using BLAKE2b, derive two shared secret from a scalarmult computation."] fn establish_session_keys (shared_secret : & x25519 :: SharedKey , client_pk : & PublicKey , server_pk : & PublicKey ,) -> Result < Digest , UnknownCryptoError > { let mut ctx = Blake2b :: new (64) ? ; ctx . update (shared_secret . unprotected_as_bytes ()) ? ; ctx . update (& client_pk . to_bytes ()) ? ; ctx . update (& server_pk . to_bytes ()) ? ; ctx . finalize () }
};
}
