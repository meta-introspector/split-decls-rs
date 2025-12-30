// Generated macro for description (function)
macro_rules! Depcrate_renderdescription {
() => {
// Module: crate::render
// Provides: {"description"}
// Dependencies: {}
pub (crate) fn description (roff : & mut Roff , cmd : & clap :: Command) { if let Some (about) = cmd . get_long_about () . or_else (| | cmd . get_about ()) { for line in about . to_string () . lines () { if line . trim () . is_empty () { roff . control ("PP" , []) ; } else { roff . text ([roman (line)]) ; } } } }
};
}
