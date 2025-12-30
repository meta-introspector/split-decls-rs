// Generated macro for span_without_enclosing_paren (function)
macro_rules! Depcrate_len_zerospan_without_enclosing_paren {
() => {
// Module: crate::len_zero
// Provides: {"span_without_enclosing_paren"}
// Dependencies: {}
fn span_without_enclosing_paren (cx : & LateContext < '_ > , span : Span) -> Span { let Some (snippet) = span . get_source_text (cx) else { return span ; } ; if has_enclosing_paren (snippet) { let source_map = cx . tcx . sess . source_map () ; let left_paren = source_map . start_point (span) ; let right_parent = source_map . end_point (span) ; left_paren . between (right_parent) } else { span } }
};
}
