macro_rules! deps {
    () => {
        AssertUnmoved!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < R : AsyncBufRead > AsyncBufRead for AssertUnmoved < R > { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { self . poll_with (| r | r . poll_fill_buf (cx)) } fn consume (self : Pin < & mut Self > , amt : usize) { self . poll_with (| r | r . consume (amt)) } }
    };
}

impl_80!()