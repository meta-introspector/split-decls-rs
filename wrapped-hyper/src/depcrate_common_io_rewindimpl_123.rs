// Generated macro for impl_123 (impl)
macro_rules! Depcrate_common_io_rewindimpl_123 {
() => {
// Module: crate::common::io::rewind
// Provides: {"impl_123"}
// Dependencies: {}
impl < T > Write for Rewind < T > where T : Write + Unpin , { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . inner) . poll_write (cx , buf) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . inner) . poll_write_vectored (cx , bufs) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . inner) . poll_flush (cx) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . inner) . poll_shutdown (cx) } fn is_write_vectored (& self) -> bool { self . inner . is_write_vectored () } }
};
}
