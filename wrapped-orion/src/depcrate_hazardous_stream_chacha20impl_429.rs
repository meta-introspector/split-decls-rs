// Generated macro for impl_429 (impl)
macro_rules! Depcrate_hazardous_stream_chacha20impl_429 {
() => {
// Module: crate::hazardous::stream::chacha20
// Provides: {"impl_429"}
// Dependencies: {}
impl Drop for ChaCha20 { fn drop (& mut self) { self . state . iter_mut () . zeroize () ; } }
};
}
