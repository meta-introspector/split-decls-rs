macro_rules! low_bits_of_byte {
    () => {
        # [inline] fn low_bits_of_byte (byte : u8) -> u8 { byte & ! CONTINUATION_BIT }
    };
}

low_bits_of_byte!();