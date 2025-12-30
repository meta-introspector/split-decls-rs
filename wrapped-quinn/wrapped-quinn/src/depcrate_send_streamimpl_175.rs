// Generated macro for impl_175 (impl)
macro_rules! Depcrate_send_streamimpl_175 {
() => {
// Module: crate::send_stream
// Provides: {"impl_175"}
// Dependencies: {}
impl Drop for SendStream { fn drop (& mut self) { let mut conn = self . conn . state . lock ("SendStream::drop") ; conn . blocked_writers . remove (& self . stream) ; if conn . error . is_some () || (self . is_0rtt && conn . check_0rtt () . is_err ()) { return ; } match conn . inner . send_stream (self . stream) . finish () { Ok (()) => conn . wake () , Err (FinishError :: Stopped (reason)) => { if conn . inner . send_stream (self . stream) . reset (reason) . is_ok () { conn . wake () ; } } Err (FinishError :: ClosedStream) => { } } } }
};
}
