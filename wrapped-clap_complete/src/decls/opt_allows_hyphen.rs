macro_rules! deps {
    () => {
        ParseState!();
    };
}

macro_rules! opt_allows_hyphen {
    () => {
        deps!();
        fn opt_allows_hyphen (state : & ParseState < '_ > , arg : & clap_lex :: ParsedArg < '_ >) -> bool { let val = arg . to_value_os () ; if val . starts_with ("-") { if let ParseState :: Opt ((opt , _)) = state { return opt . is_allow_hyphen_values_set () ; } } false }
    };
}

opt_allows_hyphen!()