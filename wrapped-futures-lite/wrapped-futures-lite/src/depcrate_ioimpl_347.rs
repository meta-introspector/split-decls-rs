// Generated macro for impl_347 (impl)
macro_rules! Depcrate_ioimpl_347 {
() => {
// Module: crate::io
// Provides: {"impl_347"}
// Dependencies: {}
impl < W : AsyncWrite + Unpin + ? Sized > Future for WriteFuture < '_ , W > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let buf = self . buf ; Pin :: new (& mut * self . writer) . poll_write (cx , buf) } }
};
}
