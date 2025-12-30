// Generated macro for span_has_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksspan_has_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"span_has_safety_comment"}
// Dependencies: {}
fn span_has_safety_comment (cx : & LateContext < '_ > , span : Span , accept_comment_above_attributes : bool) -> bool { let source_map = cx . sess () . source_map () ; let ctxt = span . ctxt () ; if ctxt . is_root () && let Some (search_span) = get_body_search_span (cx) { if let Ok (unsafe_line) = source_map . lookup_line (span . lo ()) && let Some (body_span) = walk_span_to_context (search_span , SyntaxContext :: root ()) && let Ok (body_line) = source_map . lookup_line (body_span . lo ()) && Arc :: ptr_eq (& unsafe_line . sf , & body_line . sf) && let Some (src) = unsafe_line . sf . src . as_deref () { body_line . line < unsafe_line . line && matches ! (text_has_safety_comment (src , & unsafe_line . sf . lines () [body_line . line + 1 ..= unsafe_line . line] , unsafe_line . sf . start_pos , accept_comment_above_attributes ,) , HasSafetyComment :: Yes (..)) } else { true } } else { false } }
};
}
