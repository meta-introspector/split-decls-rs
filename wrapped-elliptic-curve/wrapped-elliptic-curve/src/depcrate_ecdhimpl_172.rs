// Generated macro for impl_172 (impl)
macro_rules! Depcrate_ecdhimpl_172 {
() => {
// Module: crate::ecdh
// Provides: {"impl_172"}
// Dependencies: {}
impl < C : Curve > Drop for SharedSecret < C > { fn drop (& mut self) { self . secret_bytes . zeroize () } }
};
}
