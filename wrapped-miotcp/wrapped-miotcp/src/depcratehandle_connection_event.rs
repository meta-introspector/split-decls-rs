// Generated macro for handle_connection_event (function)
macro_rules! Depcratehandle_connection_event {
() => {
// Module: crate
// Provides: {"handle_connection_event"}
// Dependencies: {}
fn handle_connection_event (registry : & Registry , connection : & mut TcpStream , event : & Event ,) -> io :: Result < EventStatus > { if event . is_writable () { match connection . write (DATA) { Ok (n) if n < DATA . len () => return Err (io :: ErrorKind :: WriteZero . into ()) , Ok (_) => { registry . reregister (connection , event . token () , Interest :: READABLE) ? } Err (ref err) if would_block (err) => { } Err (ref err) if interrupted (err) => { return handle_connection_event (registry , connection , event) } Err (err) => return Err (err) , } } if event . is_readable () { let mut connection_closed = false ; let mut received_data = vec ! [0 ; 4096] ; let mut bytes_read = 0 ; loop { match connection . read (& mut received_data [bytes_read ..]) { Ok (0) => { connection_closed = true ; break ; } Ok (n) => { bytes_read += n ; if bytes_read == received_data . len () { received_data . resize (received_data . len () + 1024 , 0) ; } } Err (ref err) if would_block (err) => break , Err (ref err) if interrupted (err) => continue , Err (err) => return Err (err) , } } if bytes_read != 0 { let received_data = & received_data [.. bytes_read] ; if let Ok (str_buf) = from_utf8 (received_data) { println ! ("Received data: {}" , str_buf . trim_end ()) ; if str_buf . trim_end () == "exit" { return Ok (EventStatus :: Exit) ; } } else { println ! ("Received (none UTF-8) data: {received_data:?}") ; } } if connection_closed { println ! ("Connection closed") ; return Ok (EventStatus :: Done) ; } } Ok (EventStatus :: Continue) }
};
}
