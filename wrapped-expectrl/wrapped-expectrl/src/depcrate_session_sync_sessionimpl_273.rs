// Generated macro for impl_273 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_273 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_273"}
// Dependencies: {}
impl < P , S > Session < P , S > where S : Read + NonBlocking , { # [doc = " Try to read in a non-blocking mode."] # [doc = ""] # [doc = " Returns [`std::io::ErrorKind::WouldBlock`]"] # [doc = " in case there's nothing to read."] pub fn try_read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . stream . try_read (buf) } # [doc = " Verifyes if stream is empty or not."] pub fn is_empty (& mut self) -> io :: Result < bool > { self . stream . is_empty () } }
};
}
