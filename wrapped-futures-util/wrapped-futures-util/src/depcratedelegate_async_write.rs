// Generated macro for delegate_async_write (macro)
macro_rules! Depcratedelegate_async_write {
() => {
// Module: crate
// Provides: {"delegate_async_write"}
// Dependencies: {}
# [cfg (feature = "io")] # [cfg (feature = "std")] macro_rules ! delegate_async_write { ($ field : ident) => { fn poll_write (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >, buf : & [u8] ,) -> core :: task :: Poll < std :: io :: Result < usize >> { self . project () .$ field . poll_write (cx , buf) } fn poll_write_vectored (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >, bufs : & [std :: io :: IoSlice <'_ >] ,) -> core :: task :: Poll < std :: io :: Result < usize >> { self . project () .$ field . poll_write_vectored (cx , bufs) } fn poll_flush (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < std :: io :: Result < () >> { self . project () .$ field . poll_flush (cx) } fn poll_close (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < std :: io :: Result < () >> { self . project () .$ field . poll_close (cx) } } ; }
};
}
