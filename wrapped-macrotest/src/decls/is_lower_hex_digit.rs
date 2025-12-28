macro_rules! is_lower_hex_digit {
    () => {
        fn is_lower_hex_digit (byte : u8) -> bool { matches ! (byte , b'0' ..= b'9' | b'a' ..= b'f') }
    };
}

is_lower_hex_digit!()