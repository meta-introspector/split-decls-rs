macro_rules! deps {
    () => {
        FxHasher!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Hasher for FxHasher { # [inline] fn write (& mut self , bytes : & [u8]) { self . write_u64 (hash_bytes (bytes)) ; } # [inline] fn write_u8 (& mut self , i : u8) { self . add_to_hash (i as usize) ; } # [inline] fn write_u16 (& mut self , i : u16) { self . add_to_hash (i as usize) ; } # [inline] fn write_u32 (& mut self , i : u32) { self . add_to_hash (i as usize) ; } # [inline] fn write_u64 (& mut self , i : u64) { self . add_to_hash (i as usize) ; # [cfg (target_pointer_width = "32")] self . add_to_hash ((i >> 32) as usize) ; } # [inline] fn write_u128 (& mut self , i : u128) { self . add_to_hash (i as usize) ; # [cfg (target_pointer_width = "32")] self . add_to_hash ((i >> 32) as usize) ; self . add_to_hash ((i >> 64) as usize) ; # [cfg (target_pointer_width = "32")] self . add_to_hash ((i >> 96) as usize) ; } # [inline] fn write_usize (& mut self , i : usize) { self . add_to_hash (i) ; } # [cfg (feature = "nightly")] # [inline] fn write_length_prefix (& mut self , _len : usize) { } # [cfg (feature = "nightly")] # [inline] fn write_str (& mut self , s : & str) { self . write (s . as_bytes ()) } # [inline] fn finish (& self) -> u64 { # [cfg (target_pointer_width = "64")] const ROTATE : u32 = 26 ; # [cfg (target_pointer_width = "32")] const ROTATE : u32 = 15 ; self . hash . rotate_left (ROTATE) as u64 } }
    };
}

impl_12!()