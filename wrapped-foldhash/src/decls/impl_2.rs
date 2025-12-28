macro_rules! deps {
    () => {
        FoldHasher!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < 'a > Hasher for FoldHasher < 'a > { # [inline (always)] fn write (& mut self , bytes : & [u8]) { let len = bytes . len () ; self . accumulator = rotate_right (self . accumulator , len as u32) ; if len <= 16 { self . accumulator = hash_bytes_short (bytes , self . accumulator , self . seeds) ; } else { unsafe { self . accumulator = hash_bytes_long (bytes , self . accumulator , self . seeds) ; } } } # [inline (always)] fn write_u8 (& mut self , i : u8) { self . write_num (i) ; } # [inline (always)] fn write_u16 (& mut self , i : u16) { self . write_num (i) ; } # [inline (always)] fn write_u32 (& mut self , i : u32) { self . write_num (i) ; } # [inline (always)] fn write_u64 (& mut self , i : u64) { self . write_num (i) ; } # [inline (always)] fn write_u128 (& mut self , i : u128) { let lo = i as u64 ; let hi = (i >> 64) as u64 ; self . accumulator = folded_multiply (lo ^ self . accumulator , hi ^ self . seeds [0]) ; } # [inline (always)] fn write_usize (& mut self , i : usize) { # [cfg (target_pointer_width = "32")] self . write_num (i as u32) ; # [cfg (target_pointer_width = "64")] self . write_num (i as u64) ; } # [cfg (feature = "nightly")] # [inline (always)] fn write_str (& mut self , s : & str) { self . write (s . as_bytes ()) } # [inline (always)] fn finish (& self) -> u64 { if self . sponge_len > 0 { let lo = self . sponge as u64 ; let hi = (self . sponge >> 64) as u64 ; folded_multiply (lo ^ self . accumulator , hi ^ self . seeds [0]) } else { self . accumulator } } }
    };
}

impl_2!()