macro_rules! is_ident_start {
    () => {
        fn is_ident_start (ch : char) -> bool { ch == '_' || ch . is_ascii_alphabetic () }
    };
}

is_ident_start!()