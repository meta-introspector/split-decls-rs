// Generated macro for impl_2304 (impl)
macro_rules! Depcrate_io_flushimpl_2304 {
() => {
// Module: crate::io::flush
// Provides: {"impl_2304"}
// Dependencies: {}
impl < W > Future for Flush < '_ , W > where W : AsyncWrite + ? Sized + Unpin , { type Output = io :: Result < () > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Pin :: new (& mut * self . writer) . poll_flush (cx) } }
};
}
