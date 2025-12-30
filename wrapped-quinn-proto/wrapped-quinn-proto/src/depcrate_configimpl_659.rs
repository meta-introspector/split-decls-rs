// Generated macro for impl_659 (impl)
macro_rules! Depcrate_configimpl_659 {
() => {
// Module: crate::config
// Provides: {"impl_659"}
// Dependencies: {}
# [cfg (any (feature = "aws-lc-rs" , feature = "ring"))] impl ServerConfig { # [doc = " Create a server config with the given [`crypto::ServerConfig`]"] # [doc = ""] # [doc = " Uses a randomized handshake token key."] pub fn with_crypto (crypto : Arc < dyn crypto :: ServerConfig >) -> Self { # [cfg (all (feature = "aws-lc-rs" , not (feature = "ring")))] use aws_lc_rs :: hkdf ; use rand :: RngCore ; # [cfg (feature = "ring")] use ring :: hkdf ; let rng = & mut rand :: rng () ; let mut master_key = [0u8 ; 64] ; rng . fill_bytes (& mut master_key) ; let master_key = hkdf :: Salt :: new (hkdf :: HKDF_SHA256 , & []) . extract (& master_key) ; Self :: new (crypto , Arc :: new (master_key)) } }
};
}
