// Generated macro for parse_type_alias_methods (function)
macro_rules! Depcrate_graphql_scalar_attrparse_type_alias_methods {
() => {
// Module: crate::graphql_scalar::attr
// Provides: {"parse_type_alias_methods"}
// Dependencies: {}
# [doc = " Parses [`Methods`] from the provided [`Attr`] for the specified type alias."] fn parse_type_alias_methods (ast : & syn :: ItemType , attr : & Attr) -> syn :: Result < Methods > { match (attr . to_output . as_deref () . cloned () , attr . from_input . as_deref () . cloned () , attr . parse_token . as_deref () . cloned () , attr . with . as_deref () . cloned () ,) { (Some (to_output) , Some (from_input) , Some (parse_token) , None) => Ok (Methods :: Custom { to_output , from_input , parse_token , }) , (to_output , from_input , parse_token , Some (module)) => Ok (Methods :: Custom { to_output : to_output . unwrap_or_else (| | parse_quote ! { # module :: to_output }) , from_input : from_input . unwrap_or_else (| | parse_quote ! { # module :: from_input }) , parse_token : parse_token . unwrap_or_else (| | ParseToken :: Custom (parse_quote ! { # module :: parse_token })) , }) , _ => Err (ERR . custom_error (ast . span () , "all the resolvers have to be provided via `with` attribute \
             argument or a combination of `to_output_with`, `from_input_with`, \
             `parse_token_with`/`parse_token` attribute arguments" ,)) , } }
};
}
