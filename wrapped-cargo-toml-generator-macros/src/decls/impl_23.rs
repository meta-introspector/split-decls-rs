macro_rules! deps {
    () => {
        InlineTable!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Parse for InlineTable { fn parse (input : ParseStream) -> Result < Self > { let content ; braced ! (content in input) ; Ok (InlineTable { items : Punctuated :: parse_terminated (& content) ? , }) } }
    };
}

impl_23!();