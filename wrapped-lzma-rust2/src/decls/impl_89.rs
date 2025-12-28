macro_rules! deps {
    () => {
        Read!();
        Result!();
        LzmaReader!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < R : Read > Read for LzmaReader < R > { fn read (& mut self , buf : & mut [u8]) -> crate :: Result < usize > { self . read_decode (buf) } }
    };
}

impl_89!()