// Generated macro for all_numbers_upto_brace (function)
macro_rules! Depcrate_doc_doc_suspicious_footnotesall_numbers_upto_brace {
() => {
// Module: crate::doc::doc_suspicious_footnotes
// Provides: {"all_numbers_upto_brace"}
// Dependencies: {}
fn all_numbers_upto_brace (text : & str , i : usize) -> Option < usize > { for (j , c) in text . as_bytes () [i ..] . iter () . copied () . enumerate () . take (64) { if c == b']' && j != 0 { return Some (i + j + 1) ; } if ! c . is_ascii_digit () || j >= 64 { break ; } } None }
};
}
