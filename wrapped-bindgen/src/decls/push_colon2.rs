macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! push_colon2 {
    () => {
        deps!();
        pub fn push_colon2 (tokens : & mut TokenStream) { match tokens . 0 . chars () . last () { Some (':') => tokens . push_str (" ::") , _ => tokens . push_str ("::") , } }
    };
}

push_colon2!();