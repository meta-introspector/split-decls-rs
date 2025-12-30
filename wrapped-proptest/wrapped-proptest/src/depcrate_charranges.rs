// Generated macro for ranges (function)
macro_rules! Depcrate_charranges {
() => {
// Module: crate::char
// Provides: {"ranges"}
// Dependencies: {}
# [doc = " Creates a `CharStrategy` which selects characters within the given ranges,"] # [doc = " all inclusive, using the default biases."] pub fn ranges (ranges : Cow < [CharRange] >) -> CharStrategy { CharStrategy { special : Cow :: Borrowed (DEFAULT_SPECIAL_CHARS) , preferred : Cow :: Borrowed (DEFAULT_PREFERRED_RANGES) , ranges , } }
};
}
