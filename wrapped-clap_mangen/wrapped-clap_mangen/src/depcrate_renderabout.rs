// Generated macro for about (function)
macro_rules! Depcrate_renderabout {
() => {
// Module: crate::render
// Provides: {"about"}
// Dependencies: {}
pub (crate) fn about (roff : & mut Roff , cmd : & clap :: Command) { let name = cmd . get_display_name () . unwrap_or_else (| | cmd . get_name ()) ; let s = match cmd . get_about () . or_else (| | cmd . get_long_about ()) { Some (about) => format ! ("{name} - {about}") , None => name . to_owned () , } ; roff . text ([roman (s)]) ; }
};
}
