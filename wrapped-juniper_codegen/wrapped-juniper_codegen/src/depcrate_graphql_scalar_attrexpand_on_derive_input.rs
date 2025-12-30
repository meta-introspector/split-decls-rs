// Generated macro for expand_on_derive_input (function)
macro_rules! Depcrate_graphql_scalar_attrexpand_on_derive_input {
() => {
// Module: crate::graphql_scalar::attr
// Provides: {"expand_on_derive_input"}
// Dependencies: {}
# [doc = " Expands `#[graphql_scalar]` macro placed on a struct, enum or union."] fn expand_on_derive_input (attrs : Vec < syn :: Attribute > , ast : syn :: DeriveInput ,) -> syn :: Result < TokenStream > { let attr = Attr :: from_attrs (["graphql_scalar" , "graphql"] , & attrs) ? ; let methods = parse_derived_methods (& ast , & attr) ? ; let scalar = scalar :: Type :: parse (attr . scalar . as_deref () , & ast . generics) ; let def = Definition { ty : TypeOrIdent :: Ident (ast . ident . clone ()) , where_clause : attr . where_clause . map_or_else (Vec :: new , | cl | cl . into_inner ()) , generics : ast . generics . clone () , methods , name : attr . name . map (SpanContainer :: into_inner) . unwrap_or_else (| | ast . ident . to_string ()) , description : attr . description . map (SpanContainer :: into_inner) , specified_by_url : attr . specified_by_url . map (SpanContainer :: into_inner) , scalar , } ; Ok (quote ! { # ast # def }) }
};
}
