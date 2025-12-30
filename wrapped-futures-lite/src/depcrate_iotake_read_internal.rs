// Generated macro for take_read_internal (function)
macro_rules! Depcrate_iotake_read_internal {
() => {
// Module: crate::io
// Provides: {"take_read_internal"}
// Dependencies: {}
fn take_read_internal < R : AsyncRead + ? Sized > (mut rd : Pin < & mut R > , cx : & mut Context < '_ > , buf : & mut [u8] , limit : & mut u64 ,) -> Poll < Result < usize > > { if * limit == 0 { return Poll :: Ready (Ok (0)) ; } let max = cmp :: min (buf . len () as u64 , * limit) as usize ; match ready ! (rd . as_mut () . poll_read (cx , & mut buf [.. max])) { Ok (n) => { * limit -= n as u64 ; Poll :: Ready (Ok (n)) } Err (e) => Poll :: Ready (Err (e)) , } }
};
}
