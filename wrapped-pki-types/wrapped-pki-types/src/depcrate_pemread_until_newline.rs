// Generated macro for read_until_newline (function)
macro_rules! Depcrate_pemread_until_newline {
() => {
// Module: crate::pem
// Provides: {"read_until_newline"}
// Dependencies: {}
# [cfg (feature = "std")] fn read_until_newline < R : io :: BufRead + ? Sized > (r : & mut R , buf : & mut Vec < u8 >) -> io :: Result < usize > { let mut read = 0 ; loop { let (done , used) = { let available = match r . fill_buf () { Ok (n) => n , Err (ref e) if e . kind () == ErrorKind :: Interrupted => continue , Err (e) => return Err (e) , } ; match available . iter () . copied () . position (| b | b == b'\n' || b == b'\r') { Some (i) => { buf . extend_from_slice (& available [..= i]) ; (true , i + 1) } None => { buf . extend_from_slice (available) ; (false , available . len ()) } } } ; r . consume (used) ; read += used ; if done || used == 0 { return Ok (read) ; } } }
};
}
