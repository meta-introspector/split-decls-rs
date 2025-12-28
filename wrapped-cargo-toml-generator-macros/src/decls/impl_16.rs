macro_rules! deps {
    () => {
        RootInput!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Parse for RootInput { fn parse (input : ParseStream) -> Result < Self > { Ok (RootInput { items : Punctuated :: parse_terminated (input) ? , }) } }
    };
}

impl_16!();