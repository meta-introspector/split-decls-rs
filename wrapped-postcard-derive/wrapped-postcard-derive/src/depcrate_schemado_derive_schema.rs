// Generated macro for do_derive_schema (function)
macro_rules! Depcrate_schemado_derive_schema {
() => {
// Module: crate::schema
// Provides: {"do_derive_schema"}
// Dependencies: {}
pub fn do_derive_schema (input : DeriveInput) -> syn :: Result < TokenStream > { let span = input . span () ; let name = & input . ident ; let mut generator = Generator :: new (& input) ? ; let generics = match generator . bound . take () { Some (bounds) => { let mut generics = input . generics ; generics . make_where_clause () . predicates . extend (bounds) ; generics } None => generator . add_trait_bounds (input . generics) , } ; let (impl_generics , ty_generics , where_clause) = generics . split_for_impl () ; let mut rename = None ; for attr in & input . attrs { if attr . path () . is_ident ("serde") { let _ : syn :: Result < () > = attr . parse_nested_meta (| meta | { if meta . path . is_ident ("rename") { rename = Some (meta . value () ? . parse :: < syn :: LitStr > () ? . value ()) ; } Ok (()) }) ; } } let ty = generator . generate_type (& input . data , span , rename . unwrap_or_else (| | name . to_string ()) ,) ? ; let postcard_schema = & generator . postcard_schema ; let expanded = quote ! { impl # impl_generics # postcard_schema :: Schema for # name # ty_generics # where_clause { const SCHEMA : &'static # postcard_schema :: schema :: NamedType = # ty ; } } ; Ok (expanded) }
};
}
