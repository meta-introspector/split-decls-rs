// Generated macro for introspect_enum (function)
macro_rules! Depcrate_parseintrospect_enum {
() => {
// Module: crate::parse
// Provides: {"introspect_enum"}
// Dependencies: {}
fn introspect_enum (item : & DataEnum , lookup : & Lookup) -> types :: Variants { item . variants . iter () . filter_map (| variant | { if is_doc_hidden (& variant . attrs) { return None ; } let fields = match & variant . fields { Fields :: Unnamed (fields) => fields . unnamed . iter () . map (| field | introspect_type (& field . ty , lookup)) . collect () , Fields :: Unit => vec ! [] , Fields :: Named (_) => panic ! ("enum representation not supported") , } ; Some ((variant . ident . to_string () , fields)) }) . collect () }
};
}
