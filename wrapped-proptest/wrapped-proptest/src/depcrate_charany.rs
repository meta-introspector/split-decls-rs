// Generated macro for any (function)
macro_rules! Depcrate_charany {
() => {
// Module: crate::char
// Provides: {"any"}
// Dependencies: {}
# [doc = " Creates a `CharStrategy` which picks from literally any character, with the"] # [doc = " default biases."] pub fn any () -> CharStrategy < 'static > { CharStrategy { special : Cow :: Borrowed (DEFAULT_SPECIAL_CHARS) , preferred : Cow :: Borrowed (DEFAULT_PREFERRED_RANGES) , ranges : Cow :: Borrowed (WHOLE_RANGE) , } }
};
}
