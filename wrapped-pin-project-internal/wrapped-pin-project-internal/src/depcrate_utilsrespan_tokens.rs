// Generated macro for respan_tokens (function)
macro_rules! Depcrate_utilsrespan_tokens {
() => {
// Module: crate::utils
// Provides: {"respan_tokens"}
// Dependencies: {}
fn respan_tokens (tokens : TokenStream , span : Span) -> TokenStream { tokens . into_iter () . map (| mut token | { token . set_span (span) ; token }) . collect () }
};
}
