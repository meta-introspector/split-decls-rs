// Generated macro for peel_parens_str (function)
macro_rules! Depcrate_collapsible_ifpeel_parens_str {
() => {
// Module: crate::collapsible_if
// Provides: {"peel_parens_str"}
// Dependencies: {}
fn peel_parens_str (snippet : & str) -> Option < (usize , & str , usize) > { let trimmed = snippet . trim () ; if ! (trimmed . starts_with ('(') && trimmed . ends_with (')')) { return None ; } let trim_start = (snippet . len () - snippet . trim_start () . len ()) + 1 ; let trim_end = (snippet . len () - snippet . trim_end () . len ()) + 1 ; let inner = snippet . get (trim_start .. snippet . len () - trim_end) ? ; Some (match peel_parens_str (inner) { None => (trim_start , inner , trim_end) , Some ((start , inner , end)) => (trim_start + start , inner , trim_end + end) , }) }
};
}
