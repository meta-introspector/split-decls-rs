// Generated macro for after_help (function)
macro_rules! Depcrate_renderafter_help {
() => {
// Module: crate::render
// Provides: {"after_help"}
// Dependencies: {}
pub (crate) fn after_help (roff : & mut Roff , cmd : & clap :: Command) { if let Some (about) = cmd . get_after_long_help () . or_else (| | cmd . get_after_help ()) { for line in about . to_string () . lines () { roff . text ([roman (line)]) ; } } }
};
}
