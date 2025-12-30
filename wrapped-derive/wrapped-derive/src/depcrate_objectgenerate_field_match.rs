// Generated macro for generate_field_match (function)
macro_rules! Depcrate_objectgenerate_field_match {
() => {
// Module: crate::object
// Provides: {"generate_field_match"}
// Dependencies: {}
fn generate_field_match (resolvers : Vec < proc_macro2 :: TokenStream > ,) -> GeneratorResult < proc_macro2 :: TokenStream > { if resolvers . is_empty () { return Ok (quote ! ()) ; } Ok (quote ! { let __field = __FieldIdent :: from_name (& ctx . item . node . name . node) ; match __field { # (# resolvers) * None => { } } }) }
};
}
