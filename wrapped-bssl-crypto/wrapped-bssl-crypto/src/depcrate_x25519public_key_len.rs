// Generated macro for PUBLIC_KEY_LEN (const)
macro_rules! Depcrate_x25519PUBLIC_KEY_LEN {
() => {
// Module: crate::x25519
// Provides: {"PUBLIC_KEY_LEN"}
// Dependencies: {}
# [doc = " Number of bytes in a public key in X25519"] pub const PUBLIC_KEY_LEN : usize = bssl_sys :: X25519_PUBLIC_VALUE_LEN as usize ;
};
}
