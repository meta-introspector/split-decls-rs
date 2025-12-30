// Generated macro for impl_59 (impl)
macro_rules! Depcrateimpl_59 {
() => {
// Module: crate
// Provides: {"impl_59"}
// Dependencies: {}
impl AsyncSeek for File { fn poll_seek (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { ready ! (self . poll_reposition (cx)) ? ; Pin :: new (self . unblock . get_mut ()) . poll_seek (cx , pos) } }
};
}
