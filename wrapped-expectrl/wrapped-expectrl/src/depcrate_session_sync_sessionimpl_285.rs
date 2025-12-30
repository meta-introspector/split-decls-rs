// Generated macro for impl_285 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_285 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_285"}
// Dependencies: {}
impl < S > TryStream < S > where S : Read , { # [doc = " The function returns a new Stream from a file."] fn new (stream : S) -> io :: Result < Self > { Ok (Self { stream : ControlledReader :: new (stream) , }) } fn flush_in_buffer (& mut self) { self . stream . flush_in_buffer () ; } }
};
}
