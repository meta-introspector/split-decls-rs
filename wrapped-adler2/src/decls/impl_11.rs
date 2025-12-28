macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Hasher for Adler32 { # [inline] fn finish (& self) -> u64 { u64 :: from (self . checksum ()) } fn write (& mut self , bytes : & [u8]) { self . write_slice (bytes) ; } }
    };
}

impl_11!();