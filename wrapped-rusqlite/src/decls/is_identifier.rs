macro_rules! is_identifier {
    () => {
        fn is_identifier (s : & str) -> bool { let chars = s . char_indices () ; for (i , ch) in chars { if i == 0 { if ! is_identifier_start (ch) { return false ; } } else if ! is_identifier_continue (ch) { return false ; } } true }
    };
}

is_identifier!();