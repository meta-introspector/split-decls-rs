// Generated macro for find_prefixed_word (function)
macro_rules! Depcrate_matcherfind_prefixed_word {
() => {
// Module: crate::matcher
// Provides: {"find_prefixed_word"}
// Dependencies: {}
fn find_prefixed_word < 'a > (string : & 'a str , prefix : & str) -> Option < & 'a str > { if let Some (prefix_start) = string . find (prefix) { let string = & string [prefix_start + prefix . len () ..] . trim_start () ; let word_end = string . find (| c : char | c . is_whitespace ()) . unwrap_or (string . len ()) ; let string = & string [.. word_end] ; Some (string) } else { None } }
};
}
