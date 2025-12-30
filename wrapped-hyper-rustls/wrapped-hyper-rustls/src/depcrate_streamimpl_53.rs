// Generated macro for impl_53 (impl)
macro_rules! Depcrate_streamimpl_53 {
() => {
// Module: crate::stream
// Provides: {"impl_53"}
// Dependencies: {}
impl < T : rt :: Read + rt :: Write + Unpin > rt :: Read for MaybeHttpsStream < T > { # [inline] fn poll_read (self : Pin < & mut Self > , cx : & mut Context , buf : rt :: ReadBufCursor < '_ > ,) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { Self :: Http (s) => Pin :: new (s) . poll_read (cx , buf) , Self :: Https (s) => Pin :: new (s) . poll_read (cx , buf) , } } }
};
}
