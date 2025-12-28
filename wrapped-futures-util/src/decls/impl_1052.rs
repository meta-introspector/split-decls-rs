macro_rules! deps {
    () => {
        Read!();
        AllowStdIo!();
    };
}

macro_rules! impl_1052 {
    () => {
        deps!();
        impl < T > io :: Read for AllowStdIo < T > where T : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . 0 . read (buf) } fn read_vectored (& mut self , bufs : & mut [IoSliceMut < '_ >]) -> io :: Result < usize > { self . 0 . read_vectored (bufs) } fn read_to_end (& mut self , buf : & mut Vec < u8 >) -> io :: Result < usize > { self . 0 . read_to_end (buf) } fn read_to_string (& mut self , buf : & mut String) -> io :: Result < usize > { self . 0 . read_to_string (buf) } fn read_exact (& mut self , buf : & mut [u8]) -> io :: Result < () > { self . 0 . read_exact (buf) } }
    };
}

impl_1052!();