// Generated macro for parse_value (function)
macro_rules! Depcrate_graphql_enum_deriveparse_value {
() => {
// Module: crate::graphql_enum::derive
// Provides: {"parse_value"}
// Dependencies: {}
# [doc = " Parses a [`ValueDefinition`] from the given Rust enum variant definition."] # [doc = ""] # [doc = " Returns [`None`] if the parsing fails, or the enum variant is ignored."] fn parse_value (v : & syn :: Variant , renaming : rename :: Policy) -> Option < ValueDefinition > { let attr = VariantAttr :: from_attrs ("graphql" , & v . attrs) . map_err (diagnostic :: emit_error) . ok () ? ; if attr . ignore . is_some () { return None ; } if ! v . fields . is_empty () { err_variant_with_fields (& v . fields) ? ; } let name = attr . name . map_or_else (| | renaming . apply (& v . ident . unraw () . to_string ()) , SpanContainer :: into_inner ,) . into_boxed_str () ; Some (ValueDefinition { ident : v . ident . clone () , name , description : attr . description . map (SpanContainer :: into_inner) , deprecated : attr . deprecated . map (SpanContainer :: into_inner) , }) }
};
}
