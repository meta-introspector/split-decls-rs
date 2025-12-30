// Generated macro for read_to_end_internal (function)
macro_rules! Depcrate_io_read_to_endread_to_end_internal {
() => {
// Module: crate::io::read_to_end
// Provides: {"read_to_end_internal"}
// Dependencies: {}
pub (super) fn read_to_end_internal < R : AsyncRead + ? Sized > (mut rd : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut Vec < u8 > , start_len : usize ,) -> Poll < io :: Result < usize > > { let mut g = Guard { len : buf . len () , buf } ; loop { if g . len == g . buf . len () { g . buf . reserve (32) ; let spare_capacity = g . buf . capacity () - g . buf . len () ; g . buf . extend (iter :: repeat (0) . take (spare_capacity)) ; } let buf = & mut g . buf [g . len ..] ; match ready ! (rd . as_mut () . poll_read (cx , buf)) { Ok (0) => return Poll :: Ready (Ok (g . len - start_len)) , Ok (n) => { assert ! (n <= buf . len ()) ; g . len += n ; } Err (e) => return Poll :: Ready (Err (e)) , } } }
};
}
