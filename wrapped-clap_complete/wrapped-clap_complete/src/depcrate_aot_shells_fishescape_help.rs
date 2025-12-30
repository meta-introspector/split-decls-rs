// Generated macro for escape_help (function)
macro_rules! Depcrate_aot_shells_fishescape_help {
() => {
// Module: crate::aot::shells::fish
// Provides: {"escape_help"}
// Dependencies: {}
fn escape_help (help : & builder :: StyledStr) -> String { escape_string (& help . to_string () . replace ('\n' , " ") , false) }
};
}
