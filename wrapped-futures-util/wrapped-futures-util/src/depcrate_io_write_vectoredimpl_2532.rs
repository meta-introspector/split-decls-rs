// Generated macro for impl_2532 (impl)
macro_rules! Depcrate_io_write_vectoredimpl_2532 {
() => {
// Module: crate::io::write_vectored
// Provides: {"impl_2532"}
// Dependencies: {}
impl < W : AsyncWrite + ? Sized + Unpin > Future for WriteVectored < '_ , '_ , W > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . writer) . poll_write_vectored (cx , this . bufs) } }
};
}
