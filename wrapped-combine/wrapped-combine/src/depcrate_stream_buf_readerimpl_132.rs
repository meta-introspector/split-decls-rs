// Generated macro for impl_132 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_132 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_132"}
// Dependencies: {}
# [cfg (feature = "tokio-02")] impl < R : tokio_02_dep :: io :: AsyncRead > tokio_02_dep :: io :: AsyncBufRead for BufReader < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let me = self . project () ; if me . buf . is_empty () { ready ! (me . inner . poll_read_buf (cx , & mut Bytes05 (me . buf))) ? ; } Poll :: Ready (Ok (& me . buf [..])) } fn consume (self : Pin < & mut Self > , amt : usize) { let me = self . project () ; me . buf . advance (amt) ; } }
};
}
