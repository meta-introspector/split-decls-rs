macro_rules! is_ident_rest {
    () => {
        fn is_ident_rest (ch : char) -> bool { is_ident_start (ch) || ch . is_ascii_digit () }
    };
}

is_ident_rest!();