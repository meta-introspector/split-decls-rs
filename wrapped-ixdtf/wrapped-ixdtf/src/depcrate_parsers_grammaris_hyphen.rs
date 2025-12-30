// Generated macro for is_hyphen (function)
macro_rules! Depcrate_parsers_grammaris_hyphen {
() => {
// Module: crate::parsers::grammar
// Provides: {"is_hyphen"}
// Dependencies: {}
# [doc = " Checks if ascii char is a hyphen. Hyphens are used as a Date separator"] # [doc = " and as a `AttributeValueComponent` separator."] # [inline] pub (crate) const fn is_hyphen (ch : u8) -> bool { ch == b'-' }
};
}
