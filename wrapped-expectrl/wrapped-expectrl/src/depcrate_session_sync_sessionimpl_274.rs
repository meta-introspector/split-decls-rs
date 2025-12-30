// Generated macro for impl_274 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_274 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_274"}
// Dependencies: {}
impl < P , S > Write for Session < P , S > where S : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . stream . write (buf) } fn flush (& mut self) -> io :: Result < () > { self . stream . flush () } fn write_vectored (& mut self , bufs : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { self . stream . write_vectored (bufs) } }
};
}
