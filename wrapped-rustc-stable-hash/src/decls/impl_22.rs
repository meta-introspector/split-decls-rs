macro_rules! deps {
    () => {
        SipHasher128!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Hasher for SipHasher128 { # [inline] fn write_u8 (& mut self , i : u8) { self . short_write (i . to_ne_bytes ()) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . short_write (i . to_ne_bytes ()) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . short_write (i . to_ne_bytes ()) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . short_write (i . to_ne_bytes ()) ; } # [inline] fn write_usize (& mut self , i : usize) { self . short_write (i . to_ne_bytes ()) ; } # [inline] fn write_i8 (& mut self , i : i8) { self . short_write ((i as u8) . to_ne_bytes ()) ; } # [inline] fn write_i16 (& mut self , i : i16) { self . short_write ((i as u16) . to_ne_bytes ()) ; } # [inline] fn write_i32 (& mut self , i : i32) { self . short_write ((i as u32) . to_ne_bytes ()) ; } # [inline] fn write_i64 (& mut self , i : i64) { self . short_write ((i as u64) . to_ne_bytes ()) ; } # [inline] fn write_isize (& mut self , i : isize) { self . short_write ((i as usize) . to_ne_bytes ()) ; } # [inline] fn write (& mut self , msg : & [u8]) { self . slice_write (msg) ; } # [cfg (feature = "nightly")] # [inline] fn write_str (& mut self , s : & str) { self . write (s . as_bytes ()) ; self . write_u8 (0xFF) ; } fn finish (& self) -> u64 { let mut buf = self . buf ; let [a , b] = unsafe { SipHasher128 :: finish128_inner (self . nbuf , & mut buf , self . state , self . processed) } ; a . wrapping_mul (3) . wrapping_add (b) } }
    };
}

impl_22!()