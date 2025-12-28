macro_rules! deps {
    () => {
        FnvHasher!();
        Hasher!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl core :: hash :: Hasher for FnvHasher { # [inline] fn write (& mut self , bytes : & [u8]) { for byte in bytes { self . state ^= u32 :: from (* byte) ; self . state = self . state . wrapping_mul (PRIME) ; } } # [inline] fn finish (& self) -> u64 { self . finish32 () . into () } }
    };
}

impl_5!()