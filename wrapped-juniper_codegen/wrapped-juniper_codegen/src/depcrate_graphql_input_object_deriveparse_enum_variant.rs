// Generated macro for parse_enum_variant (function)
macro_rules! Depcrate_graphql_input_object_deriveparse_enum_variant {
() => {
// Module: crate::graphql_input_object::derive
// Provides: {"parse_enum_variant"}
// Dependencies: {}
# [doc = " Parses a [`FieldDefinition`] from the provided enum variant definition."] # [doc = ""] # [doc = " Returns [`None`] if the parsing fails."] fn parse_enum_variant (v : & syn :: Variant , renaming : rename :: Policy , is_internal : bool ,) -> Option < FieldDefinition > { if v . fields . len () != 1 || ! matches ! (v . fields , syn :: Fields :: Unnamed (_)) { ERR . emit_custom (v . fields . span () , "enum variant must have exactly 1 unnamed field to represent `@oneOf` input object \
             field" ,) ; } let field_attr = FieldAttr :: from_attrs ("graphql" , & v . attrs) . map_err (diagnostic :: emit_error) . ok () ? ; let ignored = field_attr . ignore . is_some () ; if let Some (default) = & field_attr . default { ERR . emit_custom (default . span_ident () , if ignored { "`default` attribute argument has no meaning for ignored variants, as they are \
                 never constructed" } else { "field cannot have default value in `@oneOf` input object" } ,) ; } let ident = & v . ident ; let name = field_attr . name . map_or_else (| | { let mut name = ident . unraw () . to_string () ; if renaming != rename :: Policy :: None { name = rename :: Policy :: SnakeCase . apply (& ident . unraw () . to_string ()) ; } renaming . apply (& name) } , SpanContainer :: into_inner ,) . into_boxed_str () ; if ! is_internal && name . starts_with ("__") { ERR . no_double_underscore (v . span ()) ; } let field_ty = v . fields . iter () . next () . unwrap () . ty . clone () ; Some (FieldDefinition { ident : ident . clone () , ty : parse_quote ! { :: core :: option :: Option <# field_ty > } , default : None , name , description : field_attr . description . map (SpanContainer :: into_inner) , deprecated : field_attr . deprecated . map (SpanContainer :: into_inner) , ignored , }) }
};
}
