// Generated macro for impl_2149 (impl)
macro_rules! Depcrate_io_buf_readerimpl_2149 {
() => {
// Module: crate::io::buf_reader
// Provides: {"impl_2149"}
// Dependencies: {}
impl < R : AsyncRead > AsyncBufRead for BufReader < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let this = self . project () ; if * this . pos >= * this . cap { debug_assert ! (* this . pos == * this . cap) ; * this . cap = ready ! (this . inner . poll_read (cx , this . buffer)) ? ; * this . pos = 0 ; } Poll :: Ready (Ok (& this . buffer [* this . pos .. * this . cap])) } fn consume (self : Pin < & mut Self > , amt : usize) { * self . project () . pos = cmp :: min (self . pos + amt , self . cap) ; } }
};
}
