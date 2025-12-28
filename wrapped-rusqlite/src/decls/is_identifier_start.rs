macro_rules! is_identifier_start {
    () => {
        fn is_identifier_start (c : char) -> bool { c . is_ascii_uppercase () || c == '_' || c . is_ascii_lowercase () || c > '\x7F' }
    };
}

is_identifier_start!();