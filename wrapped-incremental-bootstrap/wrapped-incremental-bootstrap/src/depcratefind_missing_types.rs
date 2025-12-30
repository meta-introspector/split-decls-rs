// Generated macro for find_missing_types (function)
macro_rules! Depcratefind_missing_types {
() => {
// Module: crate
// Provides: {"find_missing_types"}
// Dependencies: {}
fn find_missing_types (content : & str) -> HashSet < String > { let mut missing = HashSet :: new () ; let words : Vec < & str > = content . split_whitespace () . collect () ; for window in words . windows (2) { if window [1] == "::" { let identifier = window [0] . trim_matches (| c : char | ! c . is_alphanumeric () && c != '_') ; if ! identifier . is_empty () { missing . insert (identifier . to_string ()) ; } } } for word in & words { if word . ends_with ('!') { let macro_name = word . trim_end_matches ('!') . trim_matches (| c : char | ! c . is_alphanumeric () && c != '_') ; if ! macro_name . is_empty () { missing . insert (macro_name . to_string ()) ; } } } for word in & words { let clean_word = word . trim_matches (| c : char | ! c . is_alphanumeric () && c != '_') ; if ! clean_word . is_empty () && clean_word . chars () . next () . unwrap () . is_uppercase () && clean_word . len () > 2 { missing . insert (clean_word . to_string ()) ; } } missing }
};
}
