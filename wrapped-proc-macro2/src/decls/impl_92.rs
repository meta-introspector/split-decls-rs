macro_rules! deps {
    () => {
        Punct!();
        Ident!();
        TokenTree!();
        Group!();
        Spacing!();
        Literal!();
        TokenStream!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Display for TokenStream { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let mut joint = false ; for (i , tt) in self . inner . iter () . enumerate () { if i != 0 && ! joint { write ! (f , " ") ? ; } joint = false ; match tt { TokenTree :: Group (tt) => write ! (f , "{}" , tt) , TokenTree :: Ident (tt) => write ! (f , "{}" , tt) , TokenTree :: Punct (tt) => { joint = tt . spacing () == Spacing :: Joint ; write ! (f , "{}" , tt) } TokenTree :: Literal (tt) => write ! (f , "{}" , tt) , } ? ; } Ok (()) } }
    };
}

impl_92!();