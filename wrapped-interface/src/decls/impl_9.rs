macro_rules! deps {
    () => {
        Guid!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl syn :: parse :: Parse for Guid { fn parse (cursor : syn :: parse :: ParseStream) -> syn :: Result < Self > { let string : Option < syn :: LitStr > = cursor . parse () . ok () ; Ok (Self (string)) } }
    };
}

impl_9!()