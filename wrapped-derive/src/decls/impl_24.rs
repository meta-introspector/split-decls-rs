macro_rules! deps {
    () => {
        IdentListAttribute!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Parse for IdentListAttribute { fn parse (input : ParseStream) -> Result < Self > { Ok (IdentListAttribute { idents : input . parse_terminated (Ident :: parse , Token ! [,]) ? , }) } }
    };
}

impl_24!()