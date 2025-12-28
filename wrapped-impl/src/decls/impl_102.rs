macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Parse for IdentUnraw { fn parse (input : ParseStream) -> Result < Self > { input . call (Ident :: parse_any) . map (IdentUnraw :: new) } }
    };
}

impl_102!()