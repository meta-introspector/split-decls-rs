// Generated macro for expand (function)
macro_rules! Depcrate_messageexpand {
() => {
// Module: crate::message
// Provides: {"expand"}
// Dependencies: {}
pub fn expand (ast : & syn :: DeriveInput) -> TokenStream { let item_type = { match get_attribute_type_multiple (ast , MESSAGE_ATTR) { Ok (ty) => match ty . len () { 1 => ty [0] . clone () , _ => { return syn :: Error :: new (Span :: call_site () , format ! ("#[{}(type)] takes 1 parameters, given {}" , MESSAGE_ATTR , ty . len ()) ,) . to_compile_error () } } , Err (err) => return err . to_compile_error () , } } ; let name = & ast . ident ; let (impl_generics , ty_generics , where_clause) = ast . generics . split_for_impl () ; let item_type = item_type . map (ToTokens :: into_token_stream) . unwrap_or_else (| | quote ! { () }) ; quote ! { impl # impl_generics :: actix :: Message for # name # ty_generics # where_clause { type Result = # item_type ; } } }
};
}
