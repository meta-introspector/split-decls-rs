// Generated macro for capital_word (function)
macro_rules! Depcrate_patterncapital_word {
() => {
// Module: crate::pattern
// Provides: {"capital_word"}
// Dependencies: {}
# [doc = " Applies capital pattern to a single word using graphemes."] fn capital_word (word : & str) -> String { let mut chars = word . chars () ; if let Some (c) = chars . next () { [c . to_uppercase () . collect () , chars . as_str () . to_lowercase ()] . concat () } else { String :: new () } }
};
}
