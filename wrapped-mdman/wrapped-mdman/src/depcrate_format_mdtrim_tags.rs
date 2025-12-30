// Generated macro for trim_tags (function)
macro_rules! Depcrate_format_mdtrim_tags {
() => {
// Module: crate::format::md
// Provides: {"trim_tags"}
// Dependencies: {}
fn trim_tags (s : & str) -> String { let mut in_tag = false ; let mut in_char_ref = false ; s . chars () . filter (| & ch | match ch { '<' if in_tag => panic ! ("unexpected nested tag") , '&' if in_char_ref => panic ! ("unexpected nested char ref") , '<' => { in_tag = true ; false } '&' => { in_char_ref = true ; false } '>' if in_tag => { in_tag = false ; false } ';' if in_char_ref => { in_char_ref = false ; false } _ => ! in_tag && ! in_char_ref , }) . collect () }
};
}
