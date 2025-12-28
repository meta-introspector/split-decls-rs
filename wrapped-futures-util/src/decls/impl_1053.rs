macro_rules! deps {
    () => {
        AllowStdIo!();
        Read!();
        Ready!();
    };
}

macro_rules! impl_1053 {
    () => {
        deps!();
        impl < T > AsyncRead for AllowStdIo < T > where T : io :: Read , { fn poll_read (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , buf : & mut [u8] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . read (buf)))) } fn poll_read_vectored (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , bufs : & mut [IoSliceMut < '_ >] ,) -> Poll < io :: Result < usize > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . read_vectored (bufs)))) } }
    };
}

impl_1053!()