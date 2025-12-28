macro_rules! is_symbol_like {
    () => {
        fn is_symbol_like (s : & str) -> bool { s . chars () . all (| c | { is_ascii_alphanumeric (c) || is_ascii_punctuation (c) }) }
    };
}

is_symbol_like!();