// Generated macro for impl_301 (impl)
macro_rules! Depcrate_ioimpl_301 {
() => {
// Module: crate::io
// Provides: {"impl_301"}
// Dependencies: {}
impl < R : AsyncBufRead + Unpin + ? Sized > Future for ReadLineFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , bytes , read , } = & mut * self ; read_line_internal (Pin :: new (reader) , cx , buf , bytes , read) } }
};
}
