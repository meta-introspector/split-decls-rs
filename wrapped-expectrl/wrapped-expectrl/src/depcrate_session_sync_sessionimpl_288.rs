// Generated macro for impl_288 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_288 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_288"}
// Dependencies: {}
impl < S > Write for TryStream < S > where S : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . stream . inner . get_mut () . inner . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . stream . inner . get_mut () . inner . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . stream . inner . get_mut () . inner . write_vectored (bufs) } }
};
}
