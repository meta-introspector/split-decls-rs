macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! gen_for_struct {
    () => {
        deps!();
        fn gen_for_struct (item : & Item , item_name : & Ident , generics : & Generics , fields : & [(& Field , Item)] ,) -> Result < TokenStream , syn :: Error > { let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let into_app = into_app :: gen_for_struct (item , item_name , generics) ? ; let args = args :: gen_for_struct (item , item_name , generics , fields) ? ; Ok (quote ! { # [automatically_derived] # [allow (unused_qualifications , clippy :: redundant_locals ,)] impl # impl_generics clap :: Parser for # item_name # ty_generics # where_clause { } # into_app # args }) }
    };
}

gen_for_struct!();