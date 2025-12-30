// Generated macro for read_n_bytes (function)
macro_rules! Depcrateread_n_bytes {
() => {
// Module: crate
// Provides: {"read_n_bytes"}
// Dependencies: {}
fn read_n_bytes (opts : & Options , sess : & mut Connection , conn : & mut net :: TcpStream , n : usize) { let mut bytes = [0u8 ; MAX_MESSAGE_SIZE] ; match conn . read (& mut bytes [.. n]) { Ok (count) => { println ! ("read {count:?} bytes") ; sess . read_tls (& mut io :: Cursor :: new (& mut bytes [.. count])) . expect ("read_tls not expected to fail reading from buffer") ; } Err (err) if err . kind () == io :: ErrorKind :: ConnectionReset => { } Err (err) => panic ! ("invalid read: {err}") , } ; after_read (opts , sess , conn) ; }
};
}
