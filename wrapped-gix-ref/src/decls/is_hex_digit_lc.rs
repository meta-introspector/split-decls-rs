macro_rules! is_hex_digit_lc {
    () => {
        fn is_hex_digit_lc (b : u8) -> bool { matches ! (b , b'0' ..= b'9' | b'a' ..= b'f') }
    };
}

is_hex_digit_lc!();