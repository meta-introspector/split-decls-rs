// Generated macro for SIGNATURE_LEN (const)
macro_rules! Depcrate_ed25519SIGNATURE_LEN {
() => {
// Module: crate::ed25519
// Provides: {"SIGNATURE_LEN"}
// Dependencies: {}
# [doc = " The length in bytes of an Ed25519 signature."] pub const SIGNATURE_LEN : usize = bssl_sys :: ED25519_SIGNATURE_LEN as usize ;
};
}
