// Generated macro for impl_286 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_286 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_286"}
// Dependencies: {}
impl < S > TryStream < S > { fn keep_in_buffer (& mut self , v : & [u8]) { self . stream . keep_in_buffer (v) ; } fn get_available (& mut self) -> & [u8] { self . stream . get_available () } fn consume_available (& mut self , n : usize) { self . stream . consume_available (n) } }
};
}
