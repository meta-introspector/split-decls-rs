macro_rules! deps {
    () => {
        TokenStream!();
        Delimiter!();
    };
}

macro_rules! push_group {
    () => {
        deps!();
        pub fn push_group (tokens : & mut TokenStream , delimiter : Delimiter , inner : TokenStream) { tokens . push_space () ; tokens . push (delimiter . open ()) ; tokens . combine (& inner) ; tokens . push_space () ; tokens . push (delimiter . close ()) ; }
    };
}

push_group!();