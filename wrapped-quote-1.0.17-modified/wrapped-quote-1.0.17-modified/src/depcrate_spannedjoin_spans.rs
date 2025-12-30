// Generated macro for join_spans (function)
macro_rules! Depcrate_spannedjoin_spans {
() => {
// Module: crate::spanned
// Provides: {"join_spans"}
// Dependencies: {}
fn join_spans (tokens : TokenStream) -> Span { let mut iter = tokens . into_iter () . filter_map (| tt | { let span = tt . span () ; let debug = format ! ("{:?}" , span) ; if debug . ends_with ("bytes(0..0)") { None } else { Some (span) } }) ; let first = match iter . next () { Some (span) => span , None => return Span :: call_site () , } ; iter . fold (None , | _prev , next | Some (next)) . and_then (| last | first . join (last)) . unwrap_or (first) }
};
}
