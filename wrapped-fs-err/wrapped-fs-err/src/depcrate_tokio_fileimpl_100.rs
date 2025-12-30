// Generated macro for impl_100 (impl)
macro_rules! Depcrate_tokio_fileimpl_100 {
() => {
// Module: crate::tokio::file
// Provides: {"impl_100"}
// Dependencies: {}
impl AsyncWrite for File { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_write (cx , buf)) . map_err (| err | self . error (err , ErrorKind :: Write)) ,) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_flush (cx)) . map_err (| err | self . error (err , ErrorKind :: Flush)) ,) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_shutdown (cx)) . map_err (| err | self . error (err , ErrorKind :: Flush)) ,) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_write_vectored (cx , bufs)) . map_err (| err | self . error (err , ErrorKind :: Write)) ,) } fn is_write_vectored (& self) -> bool { self . tokio . is_write_vectored () } }
};
}
