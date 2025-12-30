// Generated macro for extract_spans (function)
macro_rules! Depcrate_errorextract_spans {
() => {
// Module: crate::error
// Provides: {"extract_spans"}
// Dependencies: {}
fn extract_spans (node : & dyn ToTokens) -> Option < (Span , Span) > { let mut t = TokenStream :: new () ; node . to_tokens (& mut t) ; let mut tokens = t . into_iter () ; let start = tokens . next () . map (| t | t . span ()) ; let end = tokens . last () . map (| t | t . span ()) ; start . map (| start | (start , end . unwrap_or (start))) }
};
}
