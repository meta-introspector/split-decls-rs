macro_rules! deps {
    () => {
        Read!();
        Result!();
        Upgraded!();
        ReadBufCursor!();
    };
}

macro_rules! impl_215 {
    () => {
        deps!();
        impl Read for Upgraded { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : ReadBufCursor < '_ > ,) -> Poll < io :: Result < () > > { Pin :: new (& mut self . io) . poll_read (cx , buf) } }
    };
}

impl_215!();