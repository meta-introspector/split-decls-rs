// Generated macro for EphemeralPrivateKey (struct)
macro_rules! Depcrate_agreementEphemeralPrivateKey {
() => {
// Module: crate::agreement
// Provides: {"EphemeralPrivateKey"}
// Dependencies: {}
# [doc = " An ephemeral private key for use (only) with `agree_ephemeral`. The"] # [doc = " signature of `agree_ephemeral` ensures that an `EphemeralPrivateKey` can be"] # [doc = " used for at most one key agreement."] pub struct EphemeralPrivateKey { private_key : ec :: Seed , algorithm : & 'static Algorithm , }
};
}
