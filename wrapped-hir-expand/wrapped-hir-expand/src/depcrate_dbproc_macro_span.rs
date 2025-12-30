// Generated macro for proc_macro_span (function)
macro_rules! Depcrate_dbproc_macro_span {
() => {
// Module: crate::db
// Provides: {"proc_macro_span"}
// Dependencies: {}
fn proc_macro_span (db : & dyn ExpandDatabase , ast : AstId < ast :: Fn >) -> Span { let root = db . parse_or_expand (ast . file_id) ; let ast_id_map = & db . ast_id_map (ast . file_id) ; let span_map = & db . span_map (ast . file_id) ; let node = ast_id_map . get (ast . value) . to_node (& root) ; let range = ast :: HasName :: name (& node) . map_or_else (| | node . syntax () . text_range () , | name | name . syntax () . text_range ()) ; span_map . span_for_range (range) }
};
}
