macro_rules! deps {
    () => {
        DefaultHasher!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        # [cfg (feature = "default-hasher")] impl Hasher for DefaultHasher { forward_writes ! { write (& [u8]) , write_u8 (u8) , write_u16 (u16) , write_u32 (u32) , write_u64 (u64) , write_u128 (u128) , write_usize (usize) , write_i8 (i8) , write_i16 (i16) , write_i32 (i32) , write_i64 (i64) , write_i128 (i128) , write_isize (isize) , } # [cfg (feature = "nightly")] forward_writes ! { write_length_prefix (usize) , write_str (& str) , } # [inline (always)] fn finish (& self) -> u64 { self . inner . finish () } }
    };
}

impl_26!()