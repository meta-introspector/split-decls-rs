// Generated macro for impl_234 (impl)
macro_rules! Depcrate_replimpl_234 {
() => {
// Module: crate::repl
// Provides: {"impl_234"}
// Dependencies: {}
impl < S > Write for ReplSession < S > where S : Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { S :: write (self . get_session_mut () , buf) } fn flush (& mut self) -> io :: Result < () > { S :: flush (self . get_session_mut ()) } }
};
}
