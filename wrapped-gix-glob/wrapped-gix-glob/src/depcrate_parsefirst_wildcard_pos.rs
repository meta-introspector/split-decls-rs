// Generated macro for first_wildcard_pos (function)
macro_rules! Depcrate_parsefirst_wildcard_pos {
() => {
// Module: crate::parse
// Provides: {"first_wildcard_pos"}
// Dependencies: {}
fn first_wildcard_pos (pat : & [u8]) -> Option < usize > { pat . find_byteset (GLOB_CHARACTERS) }
};
}
