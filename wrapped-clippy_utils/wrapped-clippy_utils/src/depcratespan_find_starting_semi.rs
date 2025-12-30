// Generated macro for span_find_starting_semi (function)
macro_rules! Depcratespan_find_starting_semi {
() => {
// Module: crate
// Provides: {"span_find_starting_semi"}
// Dependencies: {}
pub fn span_find_starting_semi (sm : & SourceMap , span : Span) -> Span { sm . span_take_while (span , | & ch | ch == ' ' || ch == ';') }
};
}
