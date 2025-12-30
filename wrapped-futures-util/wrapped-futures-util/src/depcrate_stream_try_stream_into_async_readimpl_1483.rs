// Generated macro for impl_1483 (impl)
macro_rules! Depcrate_stream_try_stream_into_async_readimpl_1483 {
() => {
// Module: crate::stream::try_stream::into_async_read
// Provides: {"impl_1483"}
// Dependencies: {}
impl < St > AsyncWrite for IntoAsyncRead < St > where St : TryStream < Error = Error > + AsyncWrite , St :: Ok : AsRef < [u8] > , { fn poll_write (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8]) -> Poll < Result < usize > > { let this = self . project () ; this . stream . poll_write (cx , buf) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let this = self . project () ; this . stream . poll_flush (cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () > > { let this = self . project () ; this . stream . poll_close (cx) } }
};
}
