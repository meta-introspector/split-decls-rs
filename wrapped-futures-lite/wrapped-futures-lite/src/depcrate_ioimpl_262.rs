// Generated macro for impl_262 (impl)
macro_rules! Depcrate_ioimpl_262 {
() => {
// Module: crate::io
// Provides: {"impl_262"}
// Dependencies: {}
impl < R : AsyncRead > AsyncBufRead for BufReader < R > { fn poll_fill_buf < 'a > (self : Pin < & 'a mut Self > , cx : & mut Context < '_ >) -> Poll < Result < & 'a [u8] > > { let mut this = self . project () ; if * this . pos >= * this . cap { debug_assert ! (* this . pos == * this . cap) ; * this . cap = ready ! (this . inner . as_mut () . poll_read (cx , this . buf)) ? ; * this . pos = 0 ; } Poll :: Ready (Ok (& this . buf [* this . pos .. * this . cap])) } fn consume (self : Pin < & mut Self > , amt : usize) { let this = self . project () ; * this . pos = cmp :: min (* this . pos + amt , * this . cap) ; } }
};
}
