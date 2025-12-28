macro_rules! deps {
    () => {
        Ready!();
        Write!();
    };
}

macro_rules! delegate_async_write_to_stdio {
    () => {
        deps!();
        macro_rules ! delegate_async_write_to_stdio { () => { fn poll_write (mut self : Pin <& mut Self >, _ : & mut Context <'_ >, buf : & [u8] ,) -> Poll < io :: Result < usize >> { Poll :: Ready (io :: Write :: write (& mut self . inner , buf)) } fn poll_write_vectored (mut self : Pin <& mut Self >, _ : & mut Context <'_ >, bufs : & [IoSlice <'_ >] ,) -> Poll < io :: Result < usize >> { Poll :: Ready (io :: Write :: write_vectored (& mut self . inner , bufs)) } fn poll_flush (mut self : Pin <& mut Self >, _ : & mut Context <'_ >) -> Poll < io :: Result < () >> { Poll :: Ready (io :: Write :: flush (& mut self . inner)) } fn poll_close (self : Pin <& mut Self >, cx : & mut Context <'_ >) -> Poll < io :: Result < () >> { self . poll_flush (cx) } } ; }
    };
}

delegate_async_write_to_stdio!();