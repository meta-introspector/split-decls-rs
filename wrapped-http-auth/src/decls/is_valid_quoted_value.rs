macro_rules! is_valid_quoted_value {
    () => {
        fn is_valid_quoted_value (s : & str) -> bool { for & b in s . as_bytes () { if char_classes (b) & (C_QDTEXT | C_ESCAPABLE) == 0 { return false ; } } true }
    };
}

is_valid_quoted_value!();