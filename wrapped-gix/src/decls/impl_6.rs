macro_rules! deps {
    () => {
        Read!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < R > io :: Read for Read < R > where R : io :: Read , { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { self . inner . read (buf) } }
    };
}

impl_6!();