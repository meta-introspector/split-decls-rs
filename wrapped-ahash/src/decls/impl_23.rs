macro_rules! deps {
    () => {
        AHasherFixed!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [doc = " A specialized hasher for fixed size primitives larger than 64 bits."] # [cfg (specialize)] impl Hasher for AHasherFixed { # [inline] fn finish (& self) -> u64 { self . 0 . short_finish () } # [inline] fn write (& mut self , bytes : & [u8]) { self . 0 . write (bytes) } # [inline] fn write_u8 (& mut self , i : u8) { self . write_u64 (i as u64) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . write_u64 (i as u64) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . write_u64 (i as u64) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . 0 . write_u64 (i) ; } # [inline] fn write_u128 (& mut self , i : u128) { self . 0 . write_u128 (i) ; } # [inline] fn write_usize (& mut self , i : usize) { self . 0 . write_usize (i) ; } }
    };
}

impl_23!()