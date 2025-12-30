// Generated macro for impl_95 (impl)
macro_rules! Depcrate_kximpl_95 {
() => {
// Module: crate::kx
// Provides: {"impl_95"}
// Dependencies: {}
impl SupportedKxGroup for X25519 { fn start (& self) -> Result < StartedKeyExchange , rustls :: Error > { let priv_key = x25519_dalek :: EphemeralSecret :: random_from_rng (rand_core :: OsRng) ; Ok (StartedKeyExchange :: Single (Box :: new (KeyExchange { pub_key : (& priv_key) . into () , priv_key , }))) } fn name (& self) -> rustls :: NamedGroup { rustls :: NamedGroup :: X25519 } }
};
}
