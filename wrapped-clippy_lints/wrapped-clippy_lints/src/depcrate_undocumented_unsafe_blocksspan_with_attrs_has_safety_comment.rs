// Generated macro for span_with_attrs_has_safety_comment (function)
macro_rules! Depcrate_undocumented_unsafe_blocksspan_with_attrs_has_safety_comment {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"span_with_attrs_has_safety_comment"}
// Dependencies: {}
# [doc = " Extends `span` to also include its attributes, then checks if that span has a safety comment."] fn span_with_attrs_has_safety_comment (cx : & LateContext < '_ > , span : Span , hir_id : HirId , accept_comment_above_attributes : bool ,) -> bool { let span = if accept_comment_above_attributes { include_attrs_in_span (cx , hir_id , span) } else { span } ; span_has_safety_comment (cx , span) }
};
}
