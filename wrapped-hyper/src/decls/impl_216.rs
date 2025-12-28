macro_rules! deps {
    () => {
        Result!();
        Upgraded!();
        Write!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl Write for Upgraded { fn poll_write (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . io) . poll_write (cx , buf) } fn poll_write_vectored (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Pin :: new (& mut self . io) . poll_write_vectored (cx , bufs) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_flush (cx) } fn poll_shutdown (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_shutdown (cx) } fn is_write_vectored (& self) -> bool { self . io . is_write_vectored () } }
    };
}

impl_216!()