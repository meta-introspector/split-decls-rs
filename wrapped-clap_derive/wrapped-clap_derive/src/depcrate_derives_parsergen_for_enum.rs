// Generated macro for gen_for_enum (function)
macro_rules! Depcrate_derives_parsergen_for_enum {
() => {
// Module: crate::derives::parser
// Provides: {"gen_for_enum"}
// Dependencies: {}
fn gen_for_enum (item : & Item , item_name : & Ident , generics : & Generics , variants : & [(& Variant , Item)] ,) -> Result < TokenStream , syn :: Error > { let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let into_app = into_app :: gen_for_enum (item , item_name , generics) ? ; let subcommand = subcommand :: gen_for_enum (item , item_name , generics , variants) ? ; Ok (quote ! { # [automatically_derived] impl # impl_generics clap :: Parser for # item_name # ty_generics # where_clause { } # into_app # subcommand }) }
};
}
