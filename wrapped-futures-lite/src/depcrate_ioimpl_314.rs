// Generated macro for impl_314 (impl)
macro_rules! Depcrate_ioimpl_314 {
() => {
// Module: crate::io
// Provides: {"impl_314"}
// Dependencies: {}
impl < R : AsyncRead + Unpin + ? Sized > Future for ReadVectoredFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , bufs } = & mut * self ; Pin :: new (reader) . poll_read_vectored (cx , bufs) } }
};
}
