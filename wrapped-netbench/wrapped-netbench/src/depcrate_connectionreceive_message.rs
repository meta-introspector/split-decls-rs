// Generated macro for receive_message (function)
macro_rules! Depcrate_connectionreceive_message {
() => {
// Module: crate::connection
// Provides: {"receive_message"}
// Dependencies: {}
# [doc = " Reads n_bytes into rbuf from the given stream."] # [doc = " Make sure rbuf.len >= n_bytes"] pub fn receive_message (n_bytes : usize , stream : & mut TcpStream , rbuf : & mut [u8]) { let mut recv = 0 ; while recv < n_bytes { match stream . read (& mut rbuf [recv ..]) { Ok (n) => recv += n , Err (err) => match err . kind () { WouldBlock => { } _ => panic ! ("Error occurred while reading: {err:?}") , } , } } }
};
}
