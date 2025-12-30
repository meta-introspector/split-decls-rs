// Generated macro for parse_field (function)
macro_rules! Depcrate_graphql_object_deriveparse_field {
() => {
// Module: crate::graphql_object::derive
// Provides: {"parse_field"}
// Dependencies: {}
# [doc = " Parses a [`field::Definition`] from the given Rust struct [`syn::Field`]."] # [doc = ""] # [doc = " Returns [`None`] if parsing fails, or the struct field is ignored."] # [must_use] fn parse_field (field : & syn :: Field , renaming : & rename :: Policy) -> Option < field :: Definition > { let attr = field :: Attr :: from_attrs ("graphql" , & field . attrs) . map_err (diagnostic :: emit_error) . ok () ? ; if attr . ignore . is_some () { return None ; } let field_ident = field . ident . as_ref () . unwrap () ; let name = attr . name . as_ref () . map (| m | m . as_ref () . value ()) . unwrap_or_else (| | renaming . apply (& field_ident . unraw () . to_string ())) ; if name . starts_with ("__") { ERR . no_double_underscore (attr . name . as_ref () . map (SpanContainer :: span_ident) . unwrap_or_else (| | field_ident . span ()) ,) ; return None ; } let mut ty = field . ty . unparenthesized () . clone () ; ty . lifetimes_anonymized () ; Some (field :: Definition { name , ty , description : attr . description . map (SpanContainer :: into_inner) , deprecated : attr . deprecated . map (SpanContainer :: into_inner) , ident : field_ident . clone () , arguments : None , has_receiver : false , is_async : false , }) }
};
}
