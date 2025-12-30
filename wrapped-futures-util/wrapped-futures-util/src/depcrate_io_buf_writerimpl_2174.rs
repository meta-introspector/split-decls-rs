// Generated macro for impl_2174 (impl)
macro_rules! Depcrate_io_buf_writerimpl_2174 {
() => {
// Module: crate::io::buf_writer
// Provides: {"impl_2174"}
// Dependencies: {}
impl < W : AsyncWrite + AsyncSeek > AsyncSeek for BufWriter < W > { # [doc = " Seek to the offset, in bytes, in the underlying writer."] # [doc = ""] # [doc = " Seeking always writes out the internal buffer before seeking."] fn poll_seek (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { ready ! (self . as_mut () . flush_buf (cx)) ? ; self . project () . inner . poll_seek (cx , pos) } }
};
}
