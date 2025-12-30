// Generated macro for impl_655 (impl)
macro_rules! Depcrate_configimpl_655 {
() => {
// Module: crate::config
// Provides: {"impl_655"}
// Dependencies: {}
# [cfg (any (feature = "aws-lc-rs" , feature = "ring"))] impl Default for EndpointConfig { fn default () -> Self { # [cfg (all (feature = "aws-lc-rs" , not (feature = "ring")))] use aws_lc_rs :: hmac ; use rand :: RngCore ; # [cfg (feature = "ring")] use ring :: hmac ; let mut reset_key = [0 ; 64] ; rand :: rng () . fill_bytes (& mut reset_key) ; Self :: new (Arc :: new (hmac :: Key :: new (hmac :: HMAC_SHA256 , & reset_key))) } }
};
}
