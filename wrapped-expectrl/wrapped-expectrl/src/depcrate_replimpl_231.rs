// Generated macro for impl_231 (impl)
macro_rules! Depcrate_replimpl_231 {
() => {
// Module: crate::repl
// Provides: {"impl_231"}
// Dependencies: {}
impl < S > Termios for ReplSession < S > where S : Termios , { fn is_echo (& self) -> io :: Result < bool > { self . get_session () . is_echo () } fn set_echo (& mut self , on : bool) -> io :: Result < bool > { self . get_session_mut () . set_echo (on) } }
};
}
