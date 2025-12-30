// Generated macro for impl_2170 (impl)
macro_rules! Depcrate_io_buf_writerimpl_2170 {
() => {
// Module: crate::io::buf_writer
// Provides: {"impl_2170"}
// Dependencies: {}
impl < W : AsyncWrite > AsyncWrite for BufWriter < W > { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { if self . buf . len () + buf . len () > self . buf . capacity () { ready ! (self . as_mut () . flush_buf (cx)) ? ; } if buf . len () >= self . buf . capacity () { self . project () . inner . poll_write (cx , buf) } else { Poll :: Ready (self . project () . buf . write (buf)) } } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { let total_len = bufs . iter () . map (| b | b . len ()) . sum :: < usize > () ; if self . buf . len () + total_len > self . buf . capacity () { ready ! (self . as_mut () . flush_buf (cx)) ? ; } if total_len >= self . buf . capacity () { self . project () . inner . poll_write_vectored (cx , bufs) } else { Poll :: Ready (self . project () . buf . write_vectored (bufs)) } } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { ready ! (self . as_mut () . flush_buf (cx)) ? ; self . project () . inner . poll_flush (cx) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { ready ! (self . as_mut () . flush_buf (cx)) ? ; self . project () . inner . poll_close (cx) } }
};
}
