// Generated macro for toggle_word (function)
macro_rules! Depcrate_patterntoggle_word {
() => {
// Module: crate::pattern
// Provides: {"toggle_word"}
// Dependencies: {}
# [doc = " Applies toggle pattern to a single word using graphemes."] fn toggle_word (word : & str) -> String { let mut chars = word . chars () ; if let Some (c) = chars . next () { [c . to_lowercase () . collect () , chars . as_str () . to_uppercase ()] . concat () } else { String :: new () } }
};
}
