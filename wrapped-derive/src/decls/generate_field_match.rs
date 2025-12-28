macro_rules! deps {
    () => {
        GeneratorResult!();
    };
}

macro_rules! generate_field_match {
    () => {
        deps!();
        fn generate_field_match (resolvers : Vec < proc_macro2 :: TokenStream > ,) -> GeneratorResult < proc_macro2 :: TokenStream > { if resolvers . is_empty () { return Ok (quote ! ()) ; } Ok (quote ! { let __field = __FieldIdent :: from_name (& ctx . item . node . name . node) ; match __field { # (# resolvers) * None => { } } }) }
    };
}

generate_field_match!();