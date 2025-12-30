// Generated macro for SharedSecret (struct)
macro_rules! Depcrate_ecdhSharedSecret {
() => {
// Module: crate::ecdh
// Provides: {"SharedSecret"}
// Dependencies: {}
# [doc = " Shared secret value computed via ECDH key agreement."] pub struct SharedSecret < C : Curve > { # [doc = " Computed secret value"] secret_bytes : FieldBytes < C > , }
};
}
