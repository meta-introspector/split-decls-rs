// Generated macro for provider_from_crate_features (function)
macro_rules! Depcrate_crypto_providerprovider_from_crate_features {
() => {
// Module: crate::crypto_provider
// Provides: {"provider_from_crate_features"}
// Dependencies: {}
fn provider_from_crate_features () -> Option < CryptoProvider > { # [cfg (all (feature = "aws-lc-rs" , not (feature = "ring")))] { return Some (aws_lc_rs :: default_provider ()) ; } # [cfg (all (feature = "ring" , not (feature = "aws-lc-rs")))] { return Some (ring :: default_provider ()) ; } # [allow (unreachable_code)] None }
};
}
