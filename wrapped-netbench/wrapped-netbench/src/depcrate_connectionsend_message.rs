// Generated macro for send_message (function)
macro_rules! Depcrate_connectionsend_message {
() => {
// Module: crate::connection
// Provides: {"send_message"}
// Dependencies: {}
# [doc = " Sends first n_bytes from wbuf using the given stream."] # [doc = " Make sure wbuf.len >= n_bytes"] pub fn send_message (n_bytes : usize , stream : & mut TcpStream , wbuf : & [u8]) { let mut send = 0 ; while send < n_bytes { match stream . write (& wbuf [send ..]) { Ok (n) => send += n , Err (err) => match err . kind () { WouldBlock => { } _ => panic ! ("Error occurred while writing: {err:?}") , } , } } }
};
}
