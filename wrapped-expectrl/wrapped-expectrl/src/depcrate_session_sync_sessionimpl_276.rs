// Generated macro for impl_276 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_276 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_276"}
// Dependencies: {}
impl < P , S > BufRead for Session < P , S > where S : Read , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . stream . fill_buf () } fn consume (& mut self , amt : usize) { self . stream . consume (amt) } }
};
}
