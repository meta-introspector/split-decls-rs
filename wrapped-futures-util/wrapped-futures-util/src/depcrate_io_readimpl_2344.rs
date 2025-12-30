// Generated macro for impl_2344 (impl)
macro_rules! Depcrate_io_readimpl_2344 {
() => {
// Module: crate::io::read
// Provides: {"impl_2344"}
// Dependencies: {}
impl < R : AsyncRead + ? Sized + Unpin > Future for Read < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . reader) . poll_read (cx , this . buf) } }
};
}
