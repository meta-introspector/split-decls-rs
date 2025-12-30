// Generated macro for is_a_true_color_terminal (function)
macro_rules! Depcrate_unix_termis_a_true_color_terminal {
() => {
// Module: crate::unix_term
// Provides: {"is_a_true_color_terminal"}
// Dependencies: {}
pub (crate) fn is_a_true_color_terminal (out : & Term) -> bool { if ! is_a_color_terminal (out) { return false ; } env :: var ("COLORTERM") . is_ok_and (| term | term == "truecolor" || term == "24bit") }
};
}
