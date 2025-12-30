// Generated macro for find_by_key (function)
macro_rules! Depcrate_matcherfind_by_key {
() => {
// Module: crate::matcher
// Provides: {"find_by_key"}
// Dependencies: {}
fn find_by_key < 'a > (string : & 'a str , key : & str) -> Option < & 'a str > { let key = [key , "="] . concat () ; for line in string . lines () { if line . starts_with (& key) { return Some (line [key . len () ..] . trim_matches (| c : char | c == '"' || c . is_whitespace ())) ; } } None }
};
}
