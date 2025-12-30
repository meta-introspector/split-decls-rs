// Generated macro for impl_54 (impl)
macro_rules! Depcrate_streamimpl_54 {
() => {
// Module: crate::stream
// Provides: {"impl_54"}
// Dependencies: {}
impl < T : rt :: Write + rt :: Read + Unpin > rt :: Write for MaybeHttpsStream < T > { # [inline] fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , io :: Error > > { match Pin :: get_mut (self) { Self :: Http (s) => Pin :: new (s) . poll_write (cx , buf) , Self :: Https (s) => Pin :: new (s) . poll_write (cx , buf) , } } # [inline] fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { Self :: Http (s) => Pin :: new (s) . poll_flush (cx) , Self :: Https (s) => Pin :: new (s) . poll_flush (cx) , } } # [inline] fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { Self :: Http (s) => Pin :: new (s) . poll_shutdown (cx) , Self :: Https (s) => Pin :: new (s) . poll_shutdown (cx) , } } # [inline] fn is_write_vectored (& self) -> bool { match self { Self :: Http (s) => s . is_write_vectored () , Self :: Https (s) => s . is_write_vectored () , } } # [inline] fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < Result < usize , io :: Error > > { match Pin :: get_mut (self) { Self :: Http (s) => Pin :: new (s) . poll_write_vectored (cx , bufs) , Self :: Https (s) => Pin :: new (s) . poll_write_vectored (cx , bufs) , } } }
};
}
