// Generated macro for gen_for_struct (function)
macro_rules! Depcrate_derives_into_appgen_for_struct {
() => {
// Module: crate::derives::into_app
// Provides: {"gen_for_struct"}
// Dependencies: {}
pub (crate) fn gen_for_struct (item : & Item , item_name : & Ident , generics : & Generics ,) -> Result < TokenStream , syn :: Error > { let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let name = item . cased_name () ; let app_var = Ident :: new ("__clap_app" , Span :: call_site ()) ; let tokens = quote ! { # [allow (dead_code , unreachable_code , unused_variables , unused_braces , unused_qualifications ,)] # [allow (clippy :: style , clippy :: complexity , clippy :: pedantic , clippy :: restriction , clippy :: perf , clippy :: deprecated , clippy :: nursery , clippy :: cargo , clippy :: suspicious_else_formatting , clippy :: almost_swapped , clippy :: redundant_locals ,)] # [automatically_derived] impl # impl_generics clap :: CommandFactory for # item_name # ty_generics # where_clause { fn command <'b > () -> clap :: Command { let # app_var = clap :: Command :: new (# name) ; < Self as clap :: Args >:: augment_args (# app_var) } fn command_for_update <'b > () -> clap :: Command { let # app_var = clap :: Command :: new (# name) ; < Self as clap :: Args >:: augment_args_for_update (# app_var) } } } ; Ok (tokens) }
};
}
