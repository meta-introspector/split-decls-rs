// Generated macro for impl_337 (impl)
macro_rules! Depcrate_ioimpl_337 {
() => {
// Module: crate::io
// Provides: {"impl_337"}
// Dependencies: {}
impl < R1 : AsyncBufRead , R2 : AsyncBufRead > AsyncBufRead for Chain < R1 , R2 > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & [u8] > > { let this = self . project () ; if ! * this . done_first { match ready ! (this . first . poll_fill_buf (cx)) { Ok ([]) => * this . done_first = true , Ok (buf) => return Poll :: Ready (Ok (buf)) , Err (err) => return Poll :: Ready (Err (err)) , } } this . second . poll_fill_buf (cx) } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; if ! * this . done_first { this . first . consume (amt) } else { this . second . consume (amt) } } }
};
}
