// Generated macro for impl_130 (impl)
macro_rules! Depcrate_recv_streamimpl_130 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_130"}
// Dependencies: {}
impl Drop for RecvStream { fn drop (& mut self) { let mut conn = self . conn . state . lock ("RecvStream::drop") ; conn . blocked_readers . remove (& self . stream) ; if conn . error . is_some () || (self . is_0rtt && conn . check_0rtt () . is_err ()) { return ; } if ! self . all_data_read { let _ = conn . inner . recv_stream (self . stream) . stop (0u32 . into ()) ; conn . wake () ; } } }
};
}
