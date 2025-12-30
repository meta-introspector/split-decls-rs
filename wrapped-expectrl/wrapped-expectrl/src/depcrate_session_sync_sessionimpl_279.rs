// Generated macro for impl_279 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_279 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_279"}
// Dependencies: {}
impl < P , S > NonBlocking for Session < P , S > where S : NonBlocking , { fn set_blocking (& mut self , on : bool) -> io :: Result < () > { S :: set_blocking (self . get_stream_mut () , on) } }
};
}
