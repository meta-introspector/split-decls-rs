macro_rules! deps {
    () => {
        Seek!();
        Ready!();
        AllowStdIo!();
    };
}

macro_rules! impl_1055 {
    () => {
        deps!();
        impl < T > AsyncSeek for AllowStdIo < T > where T : io :: Seek , { fn poll_seek (mut self : Pin < & mut Self > , _ : & mut Context < '_ > , pos : SeekFrom ,) -> Poll < io :: Result < u64 > > { Poll :: Ready (Ok (try_with_interrupt ! (self . 0 . seek (pos)))) } }
    };
}

impl_1055!();