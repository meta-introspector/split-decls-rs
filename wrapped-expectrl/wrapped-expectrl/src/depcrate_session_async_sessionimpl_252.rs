// Generated macro for impl_252 (impl)
macro_rules! Depcrate_session_async_sessionimpl_252 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_252"}
// Dependencies: {}
impl < P , S > Termios for Session < P , S > where P : Termios , { fn is_echo (& self) -> io :: Result < bool > { P :: is_echo (self . get_process ()) } fn set_echo (& mut self , on : bool) -> io :: Result < bool > { P :: set_echo (self . get_process_mut () , on) } }
};
}
