macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! link_fmt {
    () => {
        deps!();
        fn link_fmt (tokens : TokenStream) -> TokenStream { let mut tokens = tokens . 0 . replacen (" ! (  " , "!(" , 1) ; tokens = tokens . replacen (" ( " , "(" , 1) ; tokens = tokens . replace (" , " , ", ") ; tokens = tokens . replace (" )" , ")") ; tokens . into () }
    };
}

link_fmt!();