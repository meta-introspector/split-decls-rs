macro_rules! deps {
    () => {
        Ready!();
        AllowStdIo!();
    };
}

macro_rules! impl_1057 {
    () => {
        deps!();
        impl < T > AsyncBufRead for AllowStdIo < T > where T : io :: BufRead , { fn poll_fill_buf (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let this : * mut Self = & mut * self as * mut _ ; Poll :: Ready (Ok (try_with_interrupt ! (unsafe { & mut * this } . 0 . fill_buf ()))) } fn consume (mut self : Pin < & mut Self > , amt : usize) { self . 0 . consume (amt) } }
    };
}

impl_1057!()