// Generated macro for impl_359 (impl)
macro_rules! Depcrate_ioimpl_359 {
() => {
// Module: crate::io
// Provides: {"impl_359"}
// Dependencies: {}
impl < W : AsyncWrite + Unpin + ? Sized > Future for CloseFuture < '_ , W > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_close (cx) } }
};
}
