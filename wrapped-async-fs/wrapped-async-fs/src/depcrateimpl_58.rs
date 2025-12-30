// Generated macro for impl_58 (impl)
macro_rules! Depcrateimpl_58 {
() => {
// Module: crate
// Provides: {"impl_58"}
// Dependencies: {}
impl AsyncWrite for File { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { ready ! (self . poll_reposition (cx)) ? ; self . is_dirty = true ; Pin :: new (self . unblock . get_mut ()) . poll_write (cx , buf) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { if self . is_dirty { ready ! (Pin :: new (self . unblock . get_mut ()) . poll_flush (cx)) ? ; self . is_dirty = false ; } Poll :: Ready (Ok (())) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (self . unblock . get_mut ()) . poll_close (cx) } }
};
}
