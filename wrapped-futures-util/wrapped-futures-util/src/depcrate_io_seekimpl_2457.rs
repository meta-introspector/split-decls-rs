// Generated macro for impl_2457 (impl)
macro_rules! Depcrate_io_seekimpl_2457 {
() => {
// Module: crate::io::seek
// Provides: {"impl_2457"}
// Dependencies: {}
impl < S : AsyncSeek + ? Sized + Unpin > Future for Seek < '_ , S > { type Output = io :: Result < u64 > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = & mut * self ; Pin :: new (& mut this . seek) . poll_seek (cx , this . pos) } }
};
}
