// Generated macro for join_spans (function)
macro_rules! Depcrate_spannedjoin_spans {
() => {
// Module: crate::spanned
// Provides: {"join_spans"}
// Dependencies: {}
fn join_spans (tokens : TokenStream) -> Span { let mut iter = tokens . into_iter () . map (| tt | tt . span ()) ; let Some (first) = iter . next () else { return Span :: call_site () ; } ; iter . fold (None , | _prev , next | Some (next)) . and_then (| last | first . join (last)) . unwrap_or (first) }
};
}
