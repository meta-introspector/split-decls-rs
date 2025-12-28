macro_rules! deps {
    () => {
        StableHasher!();
        ExtendedHasher!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < H : ExtendedHasher > Hasher for StableHasher < H > { # [doc = " Returns a combined hash."] # [doc = ""] # [doc = " For greater precision use instead [`StableHasher::finish`]."] fn finish (& self) -> u64 { Hasher :: finish (& self . state) } # [inline] fn write (& mut self , bytes : & [u8]) { self . state . write (bytes) ; } # [cfg (feature = "nightly")] # [inline] fn write_str (& mut self , s : & str) { self . state . write_str (s) ; } # [cfg (feature = "nightly")] # [inline] fn write_length_prefix (& mut self , len : usize) { self . write_usize (len) ; } # [inline] fn write_u8 (& mut self , i : u8) { self . state . write_u8 (i) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . state . short_write (i . to_le_bytes ()) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . state . short_write (i . to_le_bytes ()) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . state . short_write (i . to_le_bytes ()) ; } # [inline] fn write_u128 (& mut self , i : u128) { self . write_u64 (i as u64) ; self . write_u64 ((i >> 64) as u64) ; } # [inline] fn write_usize (& mut self , i : usize) { self . state . short_write ((i as u64) . to_le_bytes ()) ; } # [inline] fn write_i8 (& mut self , i : i8) { self . state . write_i8 (i) ; } # [inline] fn write_i16 (& mut self , i : i16) { self . state . short_write ((i as u16) . to_le_bytes ()) ; } # [inline] fn write_i32 (& mut self , i : i32) { self . state . short_write ((i as u32) . to_le_bytes ()) ; } # [inline] fn write_i64 (& mut self , i : i64) { self . state . short_write ((i as u64) . to_le_bytes ()) ; } # [inline] fn write_i128 (& mut self , i : i128) { self . state . write (& (i as u128) . to_le_bytes ()) ; } # [inline] fn write_isize (& mut self , i : isize) { let value = i as u64 ; # [cold] # [inline (never)] fn hash_value < H : ExtendedHasher > (state : & mut H , value : u64) { state . write_u8 (0xFF) ; state . short_write (value . to_le_bytes ()) ; } if value < 0xFF { self . state . write_u8 (value as u8) ; } else { hash_value (& mut self . state , value) ; } } }
    };
}

impl_34!()