// Generated macro for expand_struct (function)
macro_rules! Depcrate_graphql_union_deriveexpand_struct {
() => {
// Module: crate::graphql_union::derive
// Provides: {"expand_struct"}
// Dependencies: {}
# [doc = " Expands into generated code a `#[derive(GraphQLUnion)]` macro placed on a"] # [doc = " Rust struct."] fn expand_struct (ast : syn :: DeriveInput) -> syn :: Result < Definition > { let attr = Attr :: from_attrs ("graphql" , & ast . attrs) ? ; let struct_span = ast . span () ; let struct_ident = ast . ident ; let name = attr . name . clone () . map (SpanContainer :: into_inner) . unwrap_or_else (| | struct_ident . unraw () . to_string ()) ; if ! attr . is_internal && name . starts_with ("__") { ERR . no_double_underscore (attr . name . as_ref () . map (SpanContainer :: span_ident) . unwrap_or_else (| | struct_ident . span ()) ,) ; } let mut variants = vec ! [] ; emerge_union_variants_from_attr (& mut variants , attr . external_resolvers) ; if variants . is_empty () { ERR . emit_custom (struct_span , "expects at least one union variant") ; } if ! all_variants_different (& variants) { ERR . emit_custom (struct_span , "must have a different type for each union variant" ,) ; } diagnostic :: abort_if_dirty () ; Ok (Definition { name , ty : parse_quote ! { # struct_ident } , is_trait_object : false , description : attr . description . map (SpanContainer :: into_inner) , context : attr . context . map (SpanContainer :: into_inner) . unwrap_or_else (| | parse_quote ! { () }) , scalar : scalar :: Type :: parse (attr . scalar . as_deref () , & ast . generics) , generics : ast . generics , variants , }) }
};
}
