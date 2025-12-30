// Generated macro for impl_2521 (impl)
macro_rules! Depcrate_io_writeimpl_2521 {
() => {
// Module: crate::io::write
// Provides: {"impl_2521"}
// Dependencies: {}
impl < W : AsyncWrite + ? Sized + Unpin > Future for Write < '_ , W > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . writer) . poll_write (cx , this . buf) } }
};
}
