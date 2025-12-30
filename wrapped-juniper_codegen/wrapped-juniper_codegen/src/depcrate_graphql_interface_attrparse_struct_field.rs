// Generated macro for parse_struct_field (function)
macro_rules! Depcrate_graphql_interface_attrparse_struct_field {
() => {
// Module: crate::graphql_interface::attr
// Provides: {"parse_struct_field"}
// Dependencies: {}
# [doc = " Parses a [`field::Definition`] from the given struct field definition."] # [doc = ""] # [doc = " Returns [`None`] if the parsing fails, or the struct field is ignored."] # [must_use] fn parse_struct_field (field : & mut syn :: Field , renaming : & rename :: Policy ,) -> Option < field :: Definition > { let field_ident = field . ident . as_ref () . or_else (| | err_unnamed_field (& field)) ? ; let field_attrs = field . attrs . clone () ; field . attrs = mem :: take (& mut field . attrs) . into_iter () . filter (| attr | ! path_eq_single (attr . path () , "graphql")) . collect () ; let attr = field :: Attr :: from_attrs ("graphql" , & field_attrs) . map_err (diagnostic :: emit_error) . ok () ? ; if attr . ignore . is_some () { return None ; } let name = attr . name . as_ref () . map (| m | m . as_ref () . value ()) . unwrap_or_else (| | renaming . apply (& field_ident . unraw () . to_string ())) ; if name . starts_with ("__") { ERR . no_double_underscore (attr . name . as_ref () . map (SpanContainer :: span_ident) . unwrap_or_else (| | field_ident . span ()) ,) ; return None ; } let mut ty = field . ty . clone () ; ty . lifetimes_anonymized () ; Some (field :: Definition { name , ty , description : attr . description . map (SpanContainer :: into_inner) , deprecated : attr . deprecated . map (SpanContainer :: into_inner) , ident : field_ident . clone () , arguments : None , has_receiver : false , is_async : false , }) }
};
}
