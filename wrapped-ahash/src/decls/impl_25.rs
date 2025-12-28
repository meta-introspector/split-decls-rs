macro_rules! deps {
    () => {
        AHasherStr!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [doc = " A specialized hasher for a single string"] # [doc = " Note that the other types don't panic because the hash impl for String tacks on an unneeded call. (As does vec)"] # [cfg (specialize)] impl Hasher for AHasherStr { # [inline] fn finish (& self) -> u64 { self . 0 . finish () } # [inline] fn write (& mut self , bytes : & [u8]) { if bytes . len () > 8 { self . 0 . write (bytes) } else { let value = read_small (bytes) ; self . 0 . buffer = folded_multiply (value [0] ^ self . 0 . buffer , value [1] ^ self . 0 . extra_keys [1]) ; self . 0 . pad = self . 0 . pad . wrapping_add (bytes . len () as u64) ; } } # [inline] fn write_u8 (& mut self , _i : u8) { } # [inline] fn write_u16 (& mut self , _i : u16) { } # [inline] fn write_u32 (& mut self , _i : u32) { } # [inline] fn write_u64 (& mut self , _i : u64) { } # [inline] fn write_u128 (& mut self , _i : u128) { } # [inline] fn write_usize (& mut self , _i : usize) { } }
    };
}

impl_25!();