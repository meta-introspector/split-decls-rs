macro_rules! deps {
    () => {
        Kind!();
        Item!();
    };
}

macro_rules! lits {
    () => {
        deps!();
        fn lits (variants : & [(& Variant , Item)]) -> Result < Vec < (TokenStream , Ident) > , syn :: Error > { let mut genned = Vec :: new () ; for (variant , item) in variants { if let Kind :: Skip (_ , _) = & * item . kind () { continue ; } if ! matches ! (variant . fields , Fields :: Unit) { abort ! (variant . span () , "`#[derive(ValueEnum)]` only supports unit variants. Non-unit variants must be skipped") ; } let fields = item . field_methods () ; let deprecations = item . deprecations () ; let name = item . cased_name () ; genned . push ((quote_spanned ! { variant . span () => { # deprecations clap :: builder :: PossibleValue :: new (# name) # fields } } , variant . ident . clone () ,)) ; } Ok (genned) }
    };
}

lits!()