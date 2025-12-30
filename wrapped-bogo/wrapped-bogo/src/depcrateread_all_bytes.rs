// Generated macro for read_all_bytes (function)
macro_rules! Depcrateread_all_bytes {
() => {
// Module: crate
// Provides: {"read_all_bytes"}
// Dependencies: {}
fn read_all_bytes (opts : & Options , sess : & mut Connection , conn : & mut net :: TcpStream) { match sess . read_tls (conn) { Ok (_) => { } Err (err) if err . kind () == io :: ErrorKind :: ConnectionReset => { } Err (err) => panic ! ("invalid read: {err}") , } ; after_read (opts , sess , conn) ; }
};
}
