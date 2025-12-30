// Generated macro for is_a_color_terminal (function)
macro_rules! Depcrate_unix_termis_a_color_terminal {
() => {
// Module: crate::unix_term
// Provides: {"is_a_color_terminal"}
// Dependencies: {}
pub (crate) fn is_a_color_terminal (out : & Term) -> bool { if ! is_a_terminal (out) { return false ; } if env :: var ("NO_COLOR") . is_ok () { return false ; } match env :: var ("TERM") { Ok (term) => term != "dumb" , Err (_) => false , } }
};
}
