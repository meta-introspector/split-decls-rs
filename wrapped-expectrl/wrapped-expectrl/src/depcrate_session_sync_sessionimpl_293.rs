// Generated macro for impl_293 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_293 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_293"}
// Dependencies: {}
impl < R > ControlledReader < R > { fn keep_in_buffer (& mut self , v : & [u8]) { self . inner . get_mut () . buffer . extend (v) ; } fn get_mut (& mut self) -> & mut R { & mut self . inner . get_mut () . inner } fn get_available (& mut self) -> & [u8] { & self . inner . get_ref () . buffer } fn consume_available (& mut self , n : usize) { let _ = self . inner . get_mut () . buffer . drain (.. n) ; } }
};
}
