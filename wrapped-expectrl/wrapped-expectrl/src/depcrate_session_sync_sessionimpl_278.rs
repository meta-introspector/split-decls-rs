// Generated macro for impl_278 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_278 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_278"}
// Dependencies: {}
impl < P , S > Termios for Session < P , S > where P : Termios , { fn is_echo (& self) -> io :: Result < bool > { self . get_process () . is_echo () } fn set_echo (& mut self , on : bool) -> io :: Result < bool > { self . get_process_mut () . set_echo (on) } }
};
}
