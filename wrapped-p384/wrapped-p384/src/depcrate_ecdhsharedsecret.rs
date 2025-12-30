// Generated macro for SharedSecret (type)
macro_rules! Depcrate_ecdhSharedSecret {
() => {
// Module: crate::ecdh
// Provides: {"SharedSecret"}
// Dependencies: {}
# [doc = " Shared secret value computed via ECDH key agreement."] pub type SharedSecret = elliptic_curve :: ecdh :: SharedSecret < NistP384 > ;
};
}
