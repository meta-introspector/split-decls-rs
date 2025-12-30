// Generated macro for chars_are_similar (function)
macro_rules! Depcrate_non_expressive_nameschars_are_similar {
() => {
// Module: crate::non_expressive_names
// Provides: {"chars_are_similar"}
// Dependencies: {}
# [doc = " Return true if two characters are visually similar"] fn chars_are_similar (a : char , b : char) -> bool { a == b || SIMILAR_CHARS . contains (& (a , b)) || SIMILAR_CHARS . contains (& (b , a)) }
};
}
