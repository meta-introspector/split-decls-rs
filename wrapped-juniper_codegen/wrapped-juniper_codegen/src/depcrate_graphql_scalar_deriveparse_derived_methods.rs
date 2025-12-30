// Generated macro for parse_derived_methods (function)
macro_rules! Depcrate_graphql_scalar_deriveparse_derived_methods {
() => {
// Module: crate::graphql_scalar::derive
// Provides: {"parse_derived_methods"}
// Dependencies: {}
# [doc = " Parses [`Methods`] from the provided [`Attr`] for the specified"] # [doc = " [`syn::DeriveInput`]."] pub (super) fn parse_derived_methods (ast : & syn :: DeriveInput , attr : & Attr) -> syn :: Result < Methods > { match (attr . to_output . as_deref () . cloned () , attr . from_input . as_deref () . cloned () , attr . parse_token . as_deref () . cloned () , attr . with . as_deref () . cloned () , attr . transparent ,) { (Some (to_output) , Some (from_input) , Some (parse_token) , None , false) => { Ok (Methods :: Custom { to_output , from_input , parse_token , }) } (to_output , from_input , parse_token , module , false) => { let module = module . unwrap_or_else (| | parse_quote ! { Self }) ; Ok (Methods :: Custom { to_output : to_output . unwrap_or_else (| | parse_quote ! { # module :: to_output }) , from_input : from_input . unwrap_or_else (| | parse_quote ! { # module :: from_input }) , parse_token : parse_token . unwrap_or_else (| | ParseToken :: Custom (parse_quote ! { # module :: parse_token })) , }) } (to_output , from_input , parse_token , None , true) => { let syn :: Data :: Struct (data) = & ast . data else { return Err (ERR . custom_error (ast . span () , "`transparent` attribute argument requires exactly 1 field" ,)) ; } ; let field = match & data . fields { syn :: Fields :: Unit => Err (ERR . custom_error (ast . span () , "`transparent` attribute argument requires exactly 1 field" ,)) , syn :: Fields :: Unnamed (fields) => fields . unnamed . first () . filter (| _ | fields . unnamed . len () == 1) . cloned () . map (Field :: Unnamed) . ok_or_else (| | { ERR . custom_error (ast . span () , "`transparent` attribute argument requires \
                             exactly 1 field" ,) }) , syn :: Fields :: Named (fields) => fields . named . first () . filter (| _ | fields . named . len () == 1) . cloned () . map (Field :: Named) . ok_or_else (| | { ERR . custom_error (ast . span () , "`transparent` attribute argument requires \
                             exactly 1 field" ,) }) , } ? ; Ok (Methods :: Delegated { to_output , from_input , parse_token , field : Box :: new (field) , }) } (_ , _ , _ , Some (module) , true) => Err (ERR . custom_error (module . span () , "`with = <path>` attribute argument cannot be combined with \
             `transparent`. \
             You can specify custom resolvers with `to_output_with`, \
             `from_input_with`, `parse_token`/`parse_token_with` attribute \
             arguments and still use `transparent` for unspecified ones." ,)) , } }
};
}
