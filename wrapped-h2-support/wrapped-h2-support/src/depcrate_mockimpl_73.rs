// Generated macro for impl_73 (impl)
macro_rules! Depcrate_mockimpl_73 {
() => {
// Module: crate::mock
// Provides: {"impl_73"}
// Dependencies: {}
impl AsyncWrite for Handle { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < Result < usize , io :: Error > > { Pin :: new (self . codec . get_mut ()) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , io :: Error > > { Pin :: new (self . codec . get_mut ()) . poll_flush (cx) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Result < () , io :: Error > > { Pin :: new (self . codec . get_mut ()) . poll_shutdown (cx) } }
};
}
