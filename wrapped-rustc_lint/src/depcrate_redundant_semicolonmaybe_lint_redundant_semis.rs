// Generated macro for maybe_lint_redundant_semis (function)
macro_rules! Depcrate_redundant_semicolonmaybe_lint_redundant_semis {
() => {
// Module: crate::redundant_semicolon
// Provides: {"maybe_lint_redundant_semis"}
// Dependencies: {}
fn maybe_lint_redundant_semis (cx : & EarlyContext < '_ > , seq : & mut Option < (Span , bool) >) { if let Some ((span , multiple)) = seq . take () { if span == rustc_span :: DUMMY_SP { return ; } let suggestion = if span . from_expansion () { None } else { Some (RedundantSemicolonsSuggestion { multiple_semicolons : multiple , span }) } ; cx . emit_span_lint (REDUNDANT_SEMICOLONS , span , RedundantSemicolonsDiag { multiple , suggestion } ,) ; } }
};
}
