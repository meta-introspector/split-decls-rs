// Generated macro for close_connection (function)
macro_rules! Depcrate_connectionclose_connection {
() => {
// Module: crate::connection
// Provides: {"close_connection"}
// Dependencies: {}
pub fn close_connection (stream : & TcpStream) { stream . shutdown (Shutdown :: Both) . expect ("shutdown call failed") ; }
};
}
