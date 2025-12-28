macro_rules! deps {
    () => {
        BracketedStringList!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Parse for BracketedStringList { fn parse (input : ParseStream) -> Result < Self > { let content ; bracketed ! (content in input) ; Ok (BracketedStringList { list : Punctuated :: parse_terminated (& content) ? , }) } }
    };
}

impl_20!()