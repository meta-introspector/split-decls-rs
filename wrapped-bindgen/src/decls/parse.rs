macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        pub fn parse (tokens : & mut TokenStream , s : & str) { tokens . push_space () ; tokens . push_str (s) ; }
    };
}

parse!()