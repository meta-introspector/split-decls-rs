// Generated macro for stmt_has_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksstmt_has_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"stmt_has_safety_comment"}
// Dependencies: {}
# [doc = " Checks if the lines immediately preceding the item contain a safety comment."] fn stmt_has_safety_comment (cx : & LateContext < '_ > , span : Span , hir_id : HirId , accept_comment_above_attributes : bool ,) -> HasSafetyComment { match span_from_macro_expansion_has_safety_comment (cx , span , accept_comment_above_attributes) { HasSafetyComment :: Maybe => () , has_safety_comment => return has_safety_comment , } if span . ctxt () != SyntaxContext :: root () { return HasSafetyComment :: No ; } let comment_start = match cx . tcx . parent_hir_node (hir_id) { Node :: Block (block) => walk_span_to_context (block . span , SyntaxContext :: root ()) . map (Span :: lo) , _ => return HasSafetyComment :: Maybe , } ; let source_map = cx . sess () . source_map () ; if let Some (comment_start) = comment_start && let Ok (unsafe_line) = source_map . lookup_line (span . lo ()) && let Ok (comment_start_line) = source_map . lookup_line (comment_start) && Arc :: ptr_eq (& unsafe_line . sf , & comment_start_line . sf) && let Some (src) = unsafe_line . sf . src . as_deref () { return if comment_start_line . line >= unsafe_line . line { HasSafetyComment :: No } else { text_has_safety_comment (src , & unsafe_line . sf . lines () [comment_start_line . line + 1 ..= unsafe_line . line] , unsafe_line . sf . start_pos , accept_comment_above_attributes ,) } ; } HasSafetyComment :: Maybe }
};
}
