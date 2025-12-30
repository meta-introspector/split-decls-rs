// Generated macro for PUBLIC_KEY_LEN (const)
macro_rules! Depcrate_ed25519PUBLIC_KEY_LEN {
() => {
// Module: crate::ed25519
// Provides: {"PUBLIC_KEY_LEN"}
// Dependencies: {}
# [doc = " The length in bytes of an Ed25519 public key."] pub const PUBLIC_KEY_LEN : usize = bssl_sys :: ED25519_PUBLIC_KEY_LEN as usize ;
};
}
