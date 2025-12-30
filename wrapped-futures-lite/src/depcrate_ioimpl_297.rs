// Generated macro for impl_297 (impl)
macro_rules! Depcrate_ioimpl_297 {
() => {
// Module: crate::io
// Provides: {"impl_297"}
// Dependencies: {}
impl < R : AsyncBufRead + Unpin + ? Sized > Future for ReadUntilFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , byte , buf , read , } = & mut * self ; read_until_internal (Pin :: new (reader) , cx , * byte , buf , read) } }
};
}
