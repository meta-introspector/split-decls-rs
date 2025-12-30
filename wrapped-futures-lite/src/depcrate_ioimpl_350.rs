// Generated macro for impl_350 (impl)
macro_rules! Depcrate_ioimpl_350 {
() => {
// Module: crate::io
// Provides: {"impl_350"}
// Dependencies: {}
impl < W : AsyncWrite + Unpin + ? Sized > Future for WriteVectoredFuture < '_ , W > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let bufs = self . bufs ; Pin :: new (& mut * self . writer) . poll_write_vectored (cx , bufs) } }
};
}
