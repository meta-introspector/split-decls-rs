// Generated macro for format (function)
macro_rules! Depcrate_expand_macroformat {
() => {
// Module: crate::expand_macro
// Provides: {"format"}
// Dependencies: {}
fn format (db : & RootDatabase , kind : SyntaxKind , file_id : FileId , expanded : SyntaxNode , span_map : & SpanMap < SyntaxContext > , krate : Crate ,) -> String { let expansion = prettify_macro_expansion (db , expanded , span_map , krate) . to_string () ; _format (db , kind , file_id , & expansion) . unwrap_or (expansion) }
};
}
