// Generated macro for impl_311 (impl)
macro_rules! Depcrate_ioimpl_311 {
() => {
// Module: crate::io
// Provides: {"impl_311"}
// Dependencies: {}
impl < R : AsyncRead + Unpin + ? Sized > Future for ReadFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf } = & mut * self ; Pin :: new (reader) . poll_read (cx , buf) } }
};
}
