// Generated macro for SEED_LEN (const)
macro_rules! Depcrate_ed25519SEED_LEN {
() => {
// Module: crate::ed25519
// Provides: {"SEED_LEN"}
// Dependencies: {}
# [doc = " The length in bytes of an Ed25519 seed which is the 32-byte private key"] # [doc = " representation defined in RFC 8032."] pub const SEED_LEN : usize = (bssl_sys :: ED25519_PRIVATE_KEY_LEN - bssl_sys :: ED25519_PUBLIC_KEY_LEN) as usize ;
};
}
