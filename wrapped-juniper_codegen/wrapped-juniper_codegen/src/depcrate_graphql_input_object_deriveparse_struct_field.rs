// Generated macro for parse_struct_field (function)
macro_rules! Depcrate_graphql_input_object_deriveparse_struct_field {
() => {
// Module: crate::graphql_input_object::derive
// Provides: {"parse_struct_field"}
// Dependencies: {}
# [doc = " Parses a [`FieldDefinition`] from the provided struct field definition."] # [doc = ""] # [doc = " Returns [`None`] if the parsing fails."] fn parse_struct_field (f : & syn :: Field , renaming : rename :: Policy , is_internal : bool ,) -> Option < FieldDefinition > { let field_attr = FieldAttr :: from_attrs ("graphql" , & f . attrs) . map_err (diagnostic :: emit_error) . ok () ? ; let ident = f . ident . as_ref () . or_else (| | err_unnamed_field (f)) ? ; let name = field_attr . name . map_or_else (| | renaming . apply (& ident . unraw () . to_string ()) , SpanContainer :: into_inner ,) . into_boxed_str () ; if ! is_internal && name . starts_with ("__") { ERR . no_double_underscore (f . span ()) ; } Some (FieldDefinition { ident : ident . clone () , ty : f . ty . clone () , default : field_attr . default . map (SpanContainer :: into_inner) , name , description : field_attr . description . map (SpanContainer :: into_inner) , deprecated : field_attr . deprecated . map (SpanContainer :: into_inner) , ignored : field_attr . ignore . is_some () , }) }
};
}
