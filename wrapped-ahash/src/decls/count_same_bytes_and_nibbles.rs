macro_rules! count_same_bytes_and_nibbles {
    () => {
        fn count_same_bytes_and_nibbles (a : u64 , b : u64) -> (i32 , i32) { let mut same_byte_count = 0 ; let mut same_nibble_count = 0 ; for byte in 0 .. 8 { let ba = (a >> (8 * byte)) as u8 ; let bb = (b >> (8 * byte)) as u8 ; if ba == bb { same_byte_count += 1 ; } if ba & 0xF0u8 == bb & 0xF0u8 { same_nibble_count += 1 ; } if ba & 0x0Fu8 == bb & 0x0Fu8 { same_nibble_count += 1 ; } } (same_byte_count , same_nibble_count) }
    };
}

count_same_bytes_and_nibbles!()