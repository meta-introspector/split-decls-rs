// Generated macro for with_span (function)
macro_rules! Depcrate_helperswith_span {
() => {
// Module: crate::helpers
// Provides: {"with_span"}
// Dependencies: {}
# [doc = " Attach a [`Span`] to a [`TokenTree`]."] pub (crate) fn with_span (mut tree : TokenTree , span : Span) -> TokenTree { tree . set_span (span) ; tree }
};
}
