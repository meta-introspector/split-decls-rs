// Generated macro for impl_99 (impl)
macro_rules! Depcrate_tokio_fileimpl_99 {
() => {
// Module: crate::tokio::file
// Provides: {"impl_99"}
// Dependencies: {}
impl AsyncSeek for File { fn start_seek (mut self : Pin < & mut Self > , position : SeekFrom) -> io :: Result < () > { Pin :: new (& mut self . tokio) . start_seek (position) . map_err (| err | self . error (err , ErrorKind :: Seek)) } fn poll_complete (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < u64 > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_complete (cx)) . map_err (| err | self . error (err , ErrorKind :: Seek)) ,) } }
};
}
