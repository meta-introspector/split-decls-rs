macro_rules! deps {
    () => {
        FoldHasher!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'a > Hasher for FoldHasher < 'a > { # [inline (always)] fn write (& mut self , bytes : & [u8]) { self . inner . write (bytes) ; } # [inline (always)] fn write_u8 (& mut self , i : u8) { self . inner . write_u8 (i) ; } # [inline (always)] fn write_u16 (& mut self , i : u16) { self . inner . write_u16 (i) ; } # [inline (always)] fn write_u32 (& mut self , i : u32) { self . inner . write_u32 (i) ; } # [inline (always)] fn write_u64 (& mut self , i : u64) { self . inner . write_u64 (i) ; } # [inline (always)] fn write_u128 (& mut self , i : u128) { self . inner . write_u128 (i) ; } # [inline (always)] fn write_usize (& mut self , i : usize) { self . inner . write_usize (i) ; } # [cfg (feature = "nightly")] # [inline (always)] fn write_str (& mut self , s : & str) { self . inner . write_str (s) ; } # [inline (always)] fn finish (& self) -> u64 { folded_multiply (self . inner . finish () , ARBITRARY0) } }
    };
}

impl_17!();