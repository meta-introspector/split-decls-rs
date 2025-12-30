// Generated macro for impl_92 (impl)
macro_rules! Depcrate_kximpl_92 {
() => {
// Module: crate::kx
// Provides: {"impl_92"}
// Dependencies: {}
impl crypto :: ActiveKeyExchange for KeyExchange { fn complete (self : Box < Self > , peer : & [u8]) -> Result < crypto :: SharedSecret , rustls :: Error > { let peer_array : [u8 ; 32] = peer . try_into () . map_err (| _ | rustls :: Error :: from (PeerMisbehaved :: InvalidKeyShare)) ? ; let their_pub = x25519_dalek :: PublicKey :: from (peer_array) ; let shared_secret = self . priv_key . diffie_hellman (& their_pub) ; Ok (crypto :: SharedSecret :: from (& shared_secret . as_bytes () [..])) } fn pub_key (& self) -> & [u8] { self . pub_key . as_bytes () } fn group (& self) -> rustls :: NamedGroup { X25519 . name () } }
};
}
