macro_rules! is_identifier_continue {
    () => {
        fn is_identifier_continue (c : char) -> bool { c == '$' || c . is_ascii_digit () || c . is_ascii_uppercase () || c == '_' || c . is_ascii_lowercase () || c > '\x7F' }
    };
}

is_identifier_continue!()