// Generated macro for impl_45 (impl)
macro_rules! Depcrate_streamimpl_45 {
() => {
// Module: crate::stream
// Provides: {"impl_45"}
// Dependencies: {}
impl < T : Write + Read + Unpin > Write for MaybeHttpsStream < T > { # [inline] fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , io :: Error > > { match Pin :: get_mut (self) { MaybeHttpsStream :: Http (s) => Pin :: new (s) . poll_write (cx , buf) , MaybeHttpsStream :: Https (s) => Pin :: new (s) . poll_write (cx , buf) , } } fn poll_write_vectored (self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < Result < usize , io :: Error > > { match Pin :: get_mut (self) { MaybeHttpsStream :: Http (s) => Pin :: new (s) . poll_write_vectored (cx , bufs) , MaybeHttpsStream :: Https (s) => Pin :: new (s) . poll_write_vectored (cx , bufs) , } } fn is_write_vectored (& self) -> bool { match self { MaybeHttpsStream :: Http (s) => s . is_write_vectored () , MaybeHttpsStream :: Https (s) => s . is_write_vectored () , } } # [inline] fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { MaybeHttpsStream :: Http (s) => Pin :: new (s) . poll_flush (cx) , MaybeHttpsStream :: Https (s) => Pin :: new (s) . poll_flush (cx) , } } # [inline] fn poll_shutdown (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { match Pin :: get_mut (self) { MaybeHttpsStream :: Http (s) => Pin :: new (s) . poll_shutdown (cx) , MaybeHttpsStream :: Https (s) => Pin :: new (s) . poll_shutdown (cx) , } } }
};
}
