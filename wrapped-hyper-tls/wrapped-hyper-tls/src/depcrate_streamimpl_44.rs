// Generated macro for impl_44 (impl)
macro_rules! Depcrate_streamimpl_44 {
() => {
// Module: crate::stream
// Provides: {"impl_44"}
// Dependencies: {}
impl < T : Read + Write + Unpin > Read for MaybeHttpsStream < T > { # [inline] fn poll_read (self : Pin < & mut Self > , cx : & mut Context , buf : ReadBufCursor < '_ > ,) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { MaybeHttpsStream :: Http (s) => Pin :: new (s) . poll_read (cx , buf) , MaybeHttpsStream :: Https (s) => Pin :: new (s) . poll_read (cx , buf) , } } }
};
}
