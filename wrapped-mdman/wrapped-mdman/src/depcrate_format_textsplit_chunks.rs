// Generated macro for split_chunks (function)
macro_rules! Depcrate_format_textsplit_chunks {
() => {
// Module: crate::format::text
// Provides: {"split_chunks"}
// Dependencies: {}
# [doc = " Splits the text on whitespace."] # [doc = ""] # [doc = " Consecutive whitespace is collapsed to a single ' ', and is included as a"] # [doc = " separate element in the result."] fn split_chunks (text : & str) -> Vec < & str > { let mut result = Vec :: new () ; let mut start = 0 ; while start < text . len () { match text [start ..] . find (' ') { Some (i) => { if i != 0 { result . push (& text [start .. start + i]) ; } result . push (" ") ; match text [start + i ..] . find (| c | c != ' ') { Some (n) => { start = start + i + n ; } None => { break ; } } } None => { result . push (& text [start ..]) ; break ; } } } result }
};
}
