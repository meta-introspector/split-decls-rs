// Generated macro for impl_21 (impl)
macro_rules! Depcrate_pkcs8impl_21 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_21"}
// Dependencies: {}
impl Drop for KeypairBytes { fn drop (& mut self) { # [cfg (feature = "zeroize")] self . secret_key . zeroize () } }
};
}
