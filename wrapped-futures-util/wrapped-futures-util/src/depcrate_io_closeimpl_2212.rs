// Generated macro for impl_2212 (impl)
macro_rules! Depcrate_io_closeimpl_2212 {
() => {
// Module: crate::io::close
// Provides: {"impl_2212"}
// Dependencies: {}
impl < W : AsyncWrite + ? Sized + Unpin > Future for Close < '_ , W > { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_close (cx) } }
};
}
