macro_rules! deps {
    () => {
        ImplementAttributes!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl syn :: parse :: Parse for ImplementAttributes { fn parse (cursor : syn :: parse :: ParseStream) -> syn :: parse :: Result < Self > { let mut input = Self { agile : true , .. Default :: default () } ; while ! cursor . is_empty () { input . parse_implement (cursor) ? ; } Ok (input) } }
    };
}

impl_17!()