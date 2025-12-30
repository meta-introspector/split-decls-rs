// Generated macro for impl_325 (impl)
macro_rules! Depcrate_upgradeimpl_325 {
() => {
// Module: crate::upgrade
// Provides: {"impl_325"}
// Dependencies: {}
impl Write for Upgraded { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . io) . poll_write (cx , buf) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . io) . poll_write_vectored (cx , bufs) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_flush (cx) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_shutdown (cx) } fn is_write_vectored (& self) -> bool { self . io . is_write_vectored () } }
};
}
