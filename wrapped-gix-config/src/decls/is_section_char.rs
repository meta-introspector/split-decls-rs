macro_rules! is_section_char {
    () => {
        fn is_section_char (c : u8) -> bool { c . is_ascii_alphanumeric () || c == b'-' || c == b'.' }
    };
}

is_section_char!();