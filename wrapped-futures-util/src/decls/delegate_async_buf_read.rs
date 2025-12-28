macro_rules! delegate_async_buf_read {
    () => {
        # [cfg (feature = "io")] # [cfg (feature = "std")] macro_rules ! delegate_async_buf_read { ($ field : ident) => { fn poll_fill_buf (self : core :: pin :: Pin <& mut Self >, cx : & mut core :: task :: Context <'_ >,) -> core :: task :: Poll < std :: io :: Result <& [u8] >> { self . project () .$ field . poll_fill_buf (cx) } fn consume (self : core :: pin :: Pin <& mut Self >, amt : usize) { self . project () .$ field . consume (amt) } } ; }
    };
}

delegate_async_buf_read!()