// Generated macro for load_grammar (function)
macro_rules! Depcrate_grammarload_grammar {
() => {
// Module: crate::grammar
// Provides: {"load_grammar"}
// Dependencies: {}
# [doc = " Loads the [`Grammar`] from the book."] pub fn load_grammar (book : & Book , diag : & mut Diagnostics) -> Grammar { let mut grammar = Grammar :: default () ; for item in book . iter () { let BookItem :: Chapter (ch) = item else { continue ; } ; if ch . is_draft_chapter () { continue ; } let path = ch . path . as_ref () . unwrap () . to_owned () ; for cap in GRAMMAR_RE . captures_iter (& ch . content) { let category = & cap [1] ; let input = & cap [2] ; if let Err (e) = parser :: parse_grammar (input , & mut grammar , category , & path) { warn_or_err ! (diag , "failed to parse grammar in {path:?}: {e}") ; } } } check_undefined_nt (& grammar , diag) ; check_unexpected_roots (& grammar , diag) ; grammar }
};
}
