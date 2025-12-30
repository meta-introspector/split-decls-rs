// Generated macro for span_from_macro_expansion_has_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksspan_from_macro_expansion_has_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"span_from_macro_expansion_has_safety_comment"}
// Dependencies: {}
fn span_from_macro_expansion_has_safety_comment (cx : & LateContext < '_ > , span : Span , accept_comment_above_attributes : bool ,) -> HasSafetyComment { let source_map = cx . sess () . source_map () ; let ctxt = span . ctxt () ; if ctxt == SyntaxContext :: root () { HasSafetyComment :: Maybe } else if let Ok (unsafe_line) = source_map . lookup_line (span . lo ()) && let Ok (macro_line) = source_map . lookup_line (ctxt . outer_expn_data () . def_site . lo ()) && Arc :: ptr_eq (& unsafe_line . sf , & macro_line . sf) && let Some (src) = unsafe_line . sf . src . as_deref () { if macro_line . line < unsafe_line . line { text_has_safety_comment (src , & unsafe_line . sf . lines () [macro_line . line + 1 ..= unsafe_line . line] , unsafe_line . sf . start_pos , accept_comment_above_attributes ,) } else { HasSafetyComment :: No } } else { HasSafetyComment :: Maybe } }
};
}
