macro_rules! Newtype {
    () => {
        struct Newtype (TokenStream) ;
    };
}

Newtype!();