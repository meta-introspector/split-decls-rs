// Generated macro for impl_356 (impl)
macro_rules! Depcrate_ioimpl_356 {
() => {
// Module: crate::io
// Provides: {"impl_356"}
// Dependencies: {}
impl < W : AsyncWrite + Unpin + ? Sized > Future for FlushFuture < '_ , W > { type Output = Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_flush (cx) } }
};
}
