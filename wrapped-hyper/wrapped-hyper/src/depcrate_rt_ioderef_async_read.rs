// Generated macro for deref_async_read (macro)
macro_rules! Depcrate_rt_ioderef_async_read {
() => {
// Module: crate::rt::io
// Provides: {"deref_async_read"}
// Dependencies: {}
macro_rules ! deref_async_read { () => { fn poll_read (mut self : Pin <& mut Self >, cx : & mut Context <'_ >, buf : ReadBufCursor <'_ >,) -> Poll < std :: io :: Result < () >> { Pin :: new (& mut ** self) . poll_read (cx , buf) } } ; }
};
}
