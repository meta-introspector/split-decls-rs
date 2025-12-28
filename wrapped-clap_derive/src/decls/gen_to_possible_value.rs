macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! gen_to_possible_value {
    () => {
        deps!();
        fn gen_to_possible_value (item : & Item , lits : & [(TokenStream , Ident)]) -> TokenStream { let (lit , variant) : (Vec < TokenStream > , Vec < Ident >) = lits . iter () . cloned () . unzip () ; let deprecations = item . deprecations () ; quote ! { fn to_possible_value <'a > (& self) -> :: std :: option :: Option < clap :: builder :: PossibleValue > { # deprecations match self { # (Self ::# variant => Some (# lit) ,) * _ => None } } } }
    };
}

gen_to_possible_value!()