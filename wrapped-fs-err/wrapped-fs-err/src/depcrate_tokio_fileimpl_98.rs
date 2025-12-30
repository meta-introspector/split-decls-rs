// Generated macro for impl_98 (impl)
macro_rules! Depcrate_tokio_fileimpl_98 {
() => {
// Module: crate::tokio::file
// Provides: {"impl_98"}
// Dependencies: {}
impl AsyncRead for File { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { Poll :: Ready (ready ! (Pin :: new (& mut self . tokio) . poll_read (cx , buf)) . map_err (| err | self . error (err , ErrorKind :: Read)) ,) } }
};
}
