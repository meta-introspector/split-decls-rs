macro_rules! deps {
    () => {
        AllowStdIo!();
        Ready!();
        Write!();
    };
}

macro_rules! impl_1051 {
    () => {
        deps!();
        impl < T > AsyncWrite for AllowStdIo < T > where T : io :: Write , { fn poll_write (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . write (buf)))) } fn poll_write_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & [IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . write_vectored (bufs)))) } fn poll_flush (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < io :: Result < () > > { try_with_interrupt ! (self . 0 . flush ()) ; Poll :: Ready (Ok (())) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < () > > { self . poll_flush (cx) } }
    };
}

impl_1051!()