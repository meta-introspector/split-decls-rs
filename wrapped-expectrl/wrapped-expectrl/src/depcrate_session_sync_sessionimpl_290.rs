// Generated macro for impl_290 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_290 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_290"}
// Dependencies: {}
impl < R > BufRead for TryStream < R > where R : Read , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . stream . inner . fill_buf () } fn consume (& mut self , amt : usize) { self . stream . inner . consume (amt) } }
};
}
