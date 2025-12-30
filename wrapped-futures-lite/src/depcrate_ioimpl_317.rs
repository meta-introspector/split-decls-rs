// Generated macro for impl_317 (impl)
macro_rules! Depcrate_ioimpl_317 {
() => {
// Module: crate::io
// Provides: {"impl_317"}
// Dependencies: {}
impl < R : AsyncRead + Unpin + ? Sized > Future for ReadToEndFuture < '_ , R > { type Output = Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , buf , start_len , } = & mut * self ; read_to_end_internal (Pin :: new (reader) , cx , buf , * start_len) } }
};
}
