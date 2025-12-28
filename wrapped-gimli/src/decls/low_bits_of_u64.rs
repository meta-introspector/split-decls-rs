macro_rules! low_bits_of_u64 {
    () => {
        # [inline] # [allow (dead_code)] fn low_bits_of_u64 (val : u64) -> u8 { let byte = val & u64 :: from (u8 :: MAX) ; low_bits_of_byte (byte as u8) }
    };
}

low_bits_of_u64!()