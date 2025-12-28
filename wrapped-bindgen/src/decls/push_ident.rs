macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! push_ident {
    () => {
        deps!();
        pub fn push_ident (tokens : & mut TokenStream , s : & str) { match tokens . 0 . chars () . last () { None | Some (':') => { } _ => tokens . 0 . push (' ') , } tokens . push_str (s) ; }
    };
}

push_ident!()