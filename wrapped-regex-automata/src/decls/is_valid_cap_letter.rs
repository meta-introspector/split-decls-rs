macro_rules! is_valid_cap_letter {
    () => {
        # [doc = " Returns true if and only if the given byte is allowed in a capture name"] # [doc = " written in non-brace form."] fn is_valid_cap_letter (b : u8) -> bool { matches ! (b , b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z' | b'_') }
    };
}

is_valid_cap_letter!();