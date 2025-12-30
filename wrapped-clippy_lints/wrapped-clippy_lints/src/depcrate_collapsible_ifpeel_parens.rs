// Generated macro for peel_parens (function)
macro_rules! Depcrate_collapsible_ifpeel_parens {
() => {
// Module: crate::collapsible_if
// Provides: {"peel_parens"}
// Dependencies: {}
# [doc = " Peel the parentheses from an `if` expression, e.g. `((if true {} else {}))`."] fn peel_parens (sm : & SourceMap , mut span : Span) -> (Span , Span , Span) { use crate :: rustc_span :: Pos ; let start = span . shrink_to_lo () ; let end = span . shrink_to_hi () ; let snippet = sm . span_to_snippet (span) . unwrap () ; if let Some ((trim_start , _ , trim_end)) = peel_parens_str (& snippet) { let mut data = span . data () ; data . lo = data . lo + BytePos :: from_usize (trim_start) ; data . hi = data . hi - BytePos :: from_usize (trim_end) ; span = data . span () ; } (start . with_hi (span . lo ()) , span , end . with_lo (span . hi ())) }
};
}
