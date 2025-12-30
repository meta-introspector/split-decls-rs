// Generated macro for impl_2434 (impl)
macro_rules! Depcrate_io_read_untilimpl_2434 {
() => {
// Module: crate::io::read_until
// Provides: {"impl_2434"}
// Dependencies: {}
impl < R : AsyncBufRead + ? Sized + Unpin > Future for ReadUntil < '_ , R > { type Output = io :: Result < usize > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Self { reader , byte , buf , read } = & mut * self ; read_until_internal (Pin :: new (reader) , cx , * byte , buf , read) } }
};
}
