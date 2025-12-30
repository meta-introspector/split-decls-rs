// Generated macro for range (function)
macro_rules! Depcrate_charrange {
() => {
// Module: crate::char
// Provides: {"range"}
// Dependencies: {}
# [doc = " Creates a `CharStrategy` which selects characters within the given"] # [doc = " endpoints, inclusive, using the default biases."] pub fn range (start : char , end : char) -> CharStrategy < 'static > { CharStrategy { special : Cow :: Borrowed (DEFAULT_SPECIAL_CHARS) , preferred : Cow :: Borrowed (DEFAULT_PREFERRED_RANGES) , ranges : Cow :: Owned (vec ! [start ..= end]) , } }
};
}
