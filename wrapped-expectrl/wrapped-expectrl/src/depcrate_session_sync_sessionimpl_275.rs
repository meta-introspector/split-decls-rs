// Generated macro for impl_275 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_275 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_275"}
// Dependencies: {}
impl < P , S > Read for Session < P , S > where S : Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . stream . read (buf) } }
};
}
