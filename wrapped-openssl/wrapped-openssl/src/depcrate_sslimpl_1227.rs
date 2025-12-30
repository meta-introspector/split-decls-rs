// Generated macro for impl_1227 (impl)
macro_rules! Depcrate_sslimpl_1227 {
() => {
// Module: crate::ssl
// Provides: {"impl_1227"}
// Dependencies: {}
impl < S > Drop for SslStream < S > { fn drop (& mut self) { unsafe { ManuallyDrop :: drop (& mut self . ssl) ; ManuallyDrop :: drop (& mut self . method) ; } } }
};
}
