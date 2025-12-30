// Generated macro for impl_342 (impl)
macro_rules! Depcrate_ioimpl_342 {
() => {
// Module: crate::io
// Provides: {"impl_342"}
// Dependencies: {}
impl < S : AsyncSeek + Unpin + ? Sized > Future for SeekFuture < '_ , S > { type Output = Result < u64 > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let pos = self . pos ; Pin :: new (& mut * self . seeker) . poll_seek (cx , pos) } }
};
}
