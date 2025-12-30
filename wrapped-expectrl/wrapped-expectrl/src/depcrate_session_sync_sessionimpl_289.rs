// Generated macro for impl_289 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_289 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_289"}
// Dependencies: {}
impl < R > Read for TryStream < R > where R : Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . stream . inner . read (buf) } }
};
}
