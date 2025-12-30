// Generated macro for impl_235 (impl)
macro_rules! Depcrate_replimpl_235 {
() => {
// Module: crate::repl
// Provides: {"impl_235"}
// Dependencies: {}
impl < S > Read for ReplSession < S > where S : Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { S :: read (self . get_session_mut () , buf) } }
};
}
