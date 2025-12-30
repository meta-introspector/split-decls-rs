// Generated macro for after_read (function)
macro_rules! Depcrateafter_read {
() => {
// Module: crate
// Provides: {"after_read"}
// Dependencies: {}
fn after_read (opts : & Options , sess : & mut Connection , conn : & mut net :: TcpStream) { if let Err (err) = sess . process_new_packets () { flush (sess , conn) ; orderly_close (conn) ; handle_err (opts , err) ; } }
};
}
