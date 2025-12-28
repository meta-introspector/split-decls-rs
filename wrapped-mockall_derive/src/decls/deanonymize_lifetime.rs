macro_rules! deanonymize_lifetime {
    () => {
        fn deanonymize_lifetime (lt : & mut Lifetime) { if lt . ident == "_" { lt . ident = format_ident ! ("static") ; } }
    };
}

deanonymize_lifetime!()