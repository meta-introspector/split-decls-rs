macro_rules! is_valid_scheme_char {
    () => {
        # [doc = " Check if a character is valid in a URL scheme."] # [doc = " Valid scheme characters: alphanumeric, +, -, or ."] fn is_valid_scheme_char (c : char) -> bool { c . is_ascii_alphanumeric () || c == '+' || c == '-' || c == '.' }
    };
}

is_valid_scheme_char!();