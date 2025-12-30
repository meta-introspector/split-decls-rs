// Generated macro for impl_2355 (impl)
macro_rules! Depcrate_io_read_vectoredimpl_2355 {
() => {
// Module: crate::io::read_vectored
// Provides: {"impl_2355"}
// Dependencies: {}
impl < R : AsyncRead + ? Sized + Unpin > Future for ReadVectored < '_ , '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . reader) . poll_read_vectored (cx , this . bufs) } }
};
}
