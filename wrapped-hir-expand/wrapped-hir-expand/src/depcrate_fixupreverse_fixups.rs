// Generated macro for reverse_fixups (function)
macro_rules! Depcrate_fixupreverse_fixups {
() => {
// Module: crate::fixup
// Provides: {"reverse_fixups"}
// Dependencies: {}
pub (crate) fn reverse_fixups (tt : & mut TopSubtree , undo_info : & SyntaxFixupUndoInfo) { let Some (undo_info) = undo_info . original . as_deref () else { return } ; let undo_info = & * * undo_info ; let delimiter = tt . top_subtree_delimiter_mut () ; # [allow (deprecated)] if never ! (delimiter . close . anchor . ast_id == FIXUP_DUMMY_AST_ID || delimiter . open . anchor . ast_id == FIXUP_DUMMY_AST_ID) { let span = | file_id | Span { range : TextRange :: empty (TextSize :: new (0)) , anchor : SpanAnchor { file_id , ast_id : ROOT_ERASED_FILE_AST_ID } , ctx : SyntaxContext :: root (span :: Edition :: Edition2015) , } ; delimiter . open = span (delimiter . open . anchor . file_id) ; delimiter . close = span (delimiter . close . anchor . file_id) ; } reverse_fixups_ (tt , undo_info) ; }
};
}
