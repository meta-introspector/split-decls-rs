// Generated macro for flush (function)
macro_rules! Depcrateflush {
() => {
// Module: crate
// Provides: {"flush"}
// Dependencies: {}
fn flush (sess : & mut Connection , conn : & mut net :: TcpStream) { while sess . wants_write () { if let Err (err) = sess . write_tls (conn) { println ! ("IO error: {err:?}") ; process :: exit (0) ; } } conn . flush () . unwrap () ; }
};
}
