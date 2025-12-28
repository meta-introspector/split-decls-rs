macro_rules! deps {
    () => {
        Seek!();
        AllowStdIo!();
    };
}

macro_rules! impl_1054 {
    () => {
        deps!();
        impl < T > io :: Seek for AllowStdIo < T > where T : io :: Seek , { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { self . 0 . seek (pos) } }
    };
}

impl_1054!()