// Generated macro for agree_ephemeral_ (function)
macro_rules! Depcrate_agreementagree_ephemeral_ {
() => {
// Module: crate::agreement
// Provides: {"agree_ephemeral_"}
// Dependencies: {}
fn agree_ephemeral_ < R > (my_private_key : EphemeralPrivateKey , peer_public_key : UnparsedPublicKey < & [u8] > , kdf : impl FnOnce (& [u8]) -> R , cpu : cpu :: Features ,) -> Result < R , error :: Unspecified > { if peer_public_key . algorithm != my_private_key . algorithm { return Err (error :: Unspecified) ; } let alg = & my_private_key . algorithm ; let mut shared_key = [0u8 ; ec :: ELEM_MAX_BYTES] ; let shared_key = & mut shared_key [.. alg . curve . elem_scalar_seed_len] ; (alg . ecdh) (shared_key , & my_private_key . private_key , untrusted :: Input :: from (peer_public_key . bytes) , cpu ,) ? ; Ok (kdf (shared_key)) }
};
}
