// Generated macro for parse_with_map (function)
macro_rules! Depcrate_dbparse_with_map {
() => {
// Module: crate::db
// Provides: {"parse_with_map"}
// Dependencies: {}
pub (crate) fn parse_with_map (db : & dyn ExpandDatabase , file_id : HirFileId ,) -> (Parse < SyntaxNode > , SpanMap) { match file_id { HirFileId :: FileId (file_id) => { (db . parse (file_id) . to_syntax () , SpanMap :: RealSpanMap (db . real_span_map (file_id))) } HirFileId :: MacroFile (macro_file) => { let (parse , map) = db . parse_macro_expansion (macro_file) . value ; (parse , SpanMap :: ExpansionSpanMap (map)) } } }
};
}
