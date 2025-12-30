// Generated macro for is_horizontal_whitespace (function)
macro_rules! Depcrate_frontmatteris_horizontal_whitespace {
() => {
// Module: crate::frontmatter
// Provides: {"is_horizontal_whitespace"}
// Dependencies: {}
# [doc = " True if `c` is considered horizontal whitespace according to Rust language definition."] fn is_horizontal_whitespace (c : char) -> bool { matches ! (c , '\u{0009}' | '\u{0020}') }
};
}
