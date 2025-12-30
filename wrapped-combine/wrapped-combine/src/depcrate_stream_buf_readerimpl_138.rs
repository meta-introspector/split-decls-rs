// Generated macro for impl_138 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_138 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < R : tokio_dep :: io :: AsyncRead > tokio_dep :: io :: AsyncBufRead for BufReader < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let me = self . project () ; if me . buf . is_empty () { ready ! (tokio_read_buf (me . inner , cx , me . buf)) ? ; } Poll :: Ready (Ok (& me . buf [..])) } fn consume (self : Pin < & mut Self > , amt : usize) { let me = self . project () ; me . buf . advance (amt) ; } }
};
}
