// Generated macro for impl_329 (impl)
macro_rules! Depcrate_ioimpl_329 {
() => {
// Module: crate::io
// Provides: {"impl_329"}
// Dependencies: {}
impl < R : AsyncBufRead > AsyncBufRead for Take < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & [u8] > > { let this = self . project () ; if * this . limit == 0 { return Poll :: Ready (Ok (& [])) ; } match ready ! (this . inner . poll_fill_buf (cx)) { Ok (buf) => { let cap = cmp :: min (buf . len () as u64 , * this . limit) as usize ; Poll :: Ready (Ok (& buf [.. cap])) } Err (e) => Poll :: Ready (Err (e)) , } } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; let amt = cmp :: min (amt as u64 , * this . limit) as usize ; * this . limit -= amt as u64 ; this . inner . consume (amt) ; } }
};
}
