// Generated macro for impl_17 (impl)
macro_rules! Depcrate_pkcs8impl_17 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_17"}
// Dependencies: {}
impl Drop for KeypairBytes { fn drop (& mut self) { # [cfg (feature = "zeroize")] self . secret_key . zeroize () } }
};
}
