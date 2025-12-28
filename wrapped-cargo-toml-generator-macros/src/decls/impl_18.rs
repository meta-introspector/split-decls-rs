macro_rules! deps {
    () => {
        TomlSection!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl Parse for TomlSection { fn parse (input : ParseStream) -> Result < Self > { Ok (TomlSection { items : Punctuated :: parse_terminated (input) ? , }) } }
    };
}

impl_18!()