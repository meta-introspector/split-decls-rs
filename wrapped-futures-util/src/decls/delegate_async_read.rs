macro_rules! delegate_async_read {
    () => {
        # [cfg (feature = "io")] # [cfg (feature = "std")] macro_rules ! delegate_async_read { ($ field : ident) => { fn poll_read (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >, buf : & mut [u8] ,) -> core :: task :: Poll < std :: io :: Result < usize >> { self . project () .$ field . poll_read (cx , buf) } fn poll_read_vectored (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >, bufs : & mut [std :: io :: IoSliceMut <'_ >] ,) -> core :: task :: Poll < std :: io :: Result < usize >> { self . project () .$ field . poll_read_vectored (cx , bufs) } } ; }
    };
}

delegate_async_read!();