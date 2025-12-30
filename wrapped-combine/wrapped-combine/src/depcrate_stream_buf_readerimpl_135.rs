// Generated macro for impl_135 (impl)
macro_rules! Depcrate_stream_buf_readerimpl_135 {
() => {
// Module: crate::stream::buf_reader
// Provides: {"impl_135"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < R : tokio_03_dep :: io :: AsyncRead > tokio_03_dep :: io :: AsyncBufRead for BufReader < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let me = self . project () ; if me . buf . is_empty () { ready ! (tokio_03_read_buf (cx , me . inner , me . buf)) ? ; } Poll :: Ready (Ok (& me . buf [..])) } fn consume (self : Pin < & mut Self > , amt : usize) { let me = self . project () ; me . buf . advance (amt) ; } }
};
}
