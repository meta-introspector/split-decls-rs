// Generated macro for impl_236 (impl)
macro_rules! Depcrate_replimpl_236 {
() => {
// Module: crate::repl
// Provides: {"impl_236"}
// Dependencies: {}
impl < S > BufRead for ReplSession < S > where S : BufRead , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { S :: fill_buf (self . get_session_mut ()) } fn consume (& mut self , amt : usize) { S :: consume (self . get_session_mut () , amt) } }
};
}
