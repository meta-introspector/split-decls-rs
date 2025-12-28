macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl hash :: Hasher for Hasher { fn write (& mut self , bytes : & [u8]) { self . update (bytes) } fn finish (& self) -> u64 { u64 :: from (self . clone () . finalize ()) } }
    };
}

impl_23!();