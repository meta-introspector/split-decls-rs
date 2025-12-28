macro_rules! deps {
    () => {
        AllowStdIo!();
        Write!();
    };
}

macro_rules! impl_1050 {
    () => {
        deps!();
        impl < T > io :: Write for AllowStdIo < T > where T : io :: Write , { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . 0 . write (buf) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> io :: Result < usize > { self . 0 . write_vectored (bufs) } fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } fn write_all (& mut self , buf : & [u8]) -> io :: Result < () > { self . 0 . write_all (buf) } fn write_fmt (& mut self , fmt : fmt :: Arguments < '_ >) -> io :: Result < () > { self . 0 . write_fmt (fmt) } }
    };
}

impl_1050!()