// Generated macro for impl_72 (impl)
macro_rules! Depcrate_mockimpl_72 {
() => {
// Module: crate::mock
// Provides: {"impl_72"}
// Dependencies: {}
impl AsyncRead for Handle { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf ,) -> Poll < io :: Result < () > > { Pin :: new (self . codec . get_mut ()) . poll_read (cx , buf) } }
};
}
