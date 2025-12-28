macro_rules! deps {
    () => {
        Join!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Parse for Join { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let mut join = Self :: default () ; while ! input . is_empty () { join . fut_exprs . push (input . parse :: < Expr > () ?) ; if ! input . is_empty () { input . parse :: < Token ! [,] > () ? ; } } Ok (join) } }
    };
}

impl_3!();