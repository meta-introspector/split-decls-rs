macro_rules! is_leading_or_invalid_utf8_byte {
    () => {
        # [doc = " Returns true if and only if the given byte is either a valid leading UTF-8"] # [doc = " byte, or is otherwise an invalid byte that can never appear anywhere in a"] # [doc = " valid UTF-8 sequence."] fn is_leading_or_invalid_utf8_byte (b : u8) -> bool { (b & 0b1100_0000) != 0b1000_0000 }
    };
}

is_leading_or_invalid_utf8_byte!();