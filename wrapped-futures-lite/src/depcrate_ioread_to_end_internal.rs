// Generated macro for read_to_end_internal (function)
macro_rules! Depcrate_ioread_to_end_internal {
() => {
// Module: crate::io
// Provides: {"read_to_end_internal"}
// Dependencies: {}
fn read_to_end_internal < R : AsyncRead + ? Sized > (mut rd : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut Vec < u8 > , start_len : usize ,) -> Poll < Result < usize > > { struct Guard < 'a > { buf : & 'a mut Vec < u8 > , len : usize , } impl Drop for Guard < '_ > { fn drop (& mut self) { self . buf . resize (self . len , 0) ; } } let mut g = Guard { len : buf . len () , buf , } ; let ret ; loop { if g . len == g . buf . len () { g . buf . reserve (32) ; let capacity = g . buf . capacity () ; g . buf . resize (capacity , 0) ; } match ready ! (rd . as_mut () . poll_read (cx , & mut g . buf [g . len ..])) { Ok (0) => { ret = Poll :: Ready (Ok (g . len - start_len)) ; break ; } Ok (n) => g . len += n , Err (e) => { ret = Poll :: Ready (Err (e)) ; break ; } } } ret }
};
}
