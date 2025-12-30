// Generated macro for impl_2503 (impl)
macro_rules! Depcrate_io_takeimpl_2503 {
() => {
// Module: crate::io::take
// Provides: {"impl_2503"}
// Dependencies: {}
impl < R : AsyncBufRead > AsyncBufRead for Take < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let this = self . project () ; if * this . limit == 0 { return Poll :: Ready (Ok (& [])) ; } let buf = ready ! (this . inner . poll_fill_buf (cx) ?) ; let cap = cmp :: min (buf . len () as u64 , * this . limit) as usize ; Poll :: Ready (Ok (& buf [.. cap])) } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; let amt = cmp :: min (amt as u64 , * this . limit) as usize ; * this . limit -= amt as u64 ; this . inner . consume (amt) ; } }
};
}
